//! Recording state machine
//!
//! Manages the voice recording → transcription → image transformation flow.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};
use whis_core::{AudioRecorder, TranscriptionProvider, progressive_transcribe_cloud};

use crate::clipboard::{has_image_in_clipboard, read_image_from_clipboard, write_image_to_clipboard};
use crate::config::Config;
use crate::gemini::GeminiClient;

/// Recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Idle,
    Recording,
    Transcribing,
    Transforming,
}

/// Shared application state
pub struct AppState {
    pub recording_state: Mutex<RecordingState>,
    pub is_recording: AtomicBool,
    pub recorder: Mutex<Option<AudioRecorder>>,
    pub config: Mutex<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            recording_state: Mutex::new(RecordingState::Idle),
            is_recording: AtomicBool::new(false),
            recorder: Mutex::new(None),
            config: Mutex::new(config),
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

/// Send a notification
fn notify(app: &AppHandle, title: &str, body: &str) {
    println!("[ISI] Notification: {} - {}", title, body);
    use tauri_plugin_notification::NotificationExt;
    let result = app.notification().builder()
        .title(title)
        .body(body)
        .show();
    if let Err(e) = result {
        eprintln!("[ISI] Failed to show notification: {}", e);
    }
}

/// Send an error notification
fn notify_error(app: &AppHandle, title: &str, body: &str) {
    eprintln!("[ISI] Error notification: {} - {}", title, body);
    use tauri_plugin_notification::NotificationExt;
    let _ = app.notification().builder()
        .title(title)
        .body(body)
        .show();
}

/// Reset application state to idle after error
async fn reset_state(app: &AppHandle) {
    let state = app.state::<Arc<AppState>>();
    state.is_recording.store(false, Ordering::SeqCst);
    *state.recording_state.lock().await = RecordingState::Idle;
    update_tray_icon(app, RecordingState::Idle);
    update_tray_status(app, "ISI Voice Image - Ready");
}

/// Update tray icon tooltip
fn update_tray_status(app: &AppHandle, status: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(status));
    }
}

/// Update tray icon based on recording state
fn update_tray_icon(app: &AppHandle, state: RecordingState) {
    use tauri::image::Image;

    let icon_bytes: &[u8] = match state {
        RecordingState::Idle => include_bytes!("../icons/tray-idle.png"),
        RecordingState::Recording => include_bytes!("../icons/tray-recording.png"),
        RecordingState::Transcribing | RecordingState::Transforming => {
            include_bytes!("../icons/tray-processing.png")
        }
    };

    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(icon) = Image::from_bytes(icon_bytes) {
            let _ = tray.set_icon(Some(icon));
        }
    }
}

/// Toggle recording state - called when shortcut is pressed
pub fn toggle_recording(app: AppHandle) {
    println!("[ISI] Shortcut pressed - toggle_recording called");

    // Check if we're in manual mode
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        let state = app_clone.state::<Arc<AppState>>();
        let (is_manual, manual_prompt) = {
            let config = state.config.lock().await;
            (config.is_manual_mode(), config.manual_prompt.clone())
        };

        if is_manual {
            // Manual mode: use stored prompt directly
            println!("[ISI] Manual mode - using prompt: '{}'", manual_prompt);
            if let Err(e) = process_with_text(app_clone.clone(), &manual_prompt).await {
                eprintln!("[ISI] Error processing: {}", e);
                for cause in e.chain().skip(1) {
                    eprintln!("[ISI]   Caused by: {}", cause);
                }
                notify_error(&app_clone, "Error", &format!("{}", e));
                reset_state(&app_clone).await;
            }
        } else if state.is_recording() {
            println!("[ISI] Currently recording - stopping...");
            // Stop recording
            if let Err(e) = stop_recording_and_process(app_clone.clone()).await {
                eprintln!("[ISI] Error processing: {}", e);
                notify_error(&app_clone, "Error", &format!("{}", e));
                reset_state(&app_clone).await;
            }
        } else {
            println!("[ISI] Not recording - starting...");
            // Start recording
            if let Err(e) = start_recording(app_clone.clone()).await {
                eprintln!("[ISI] Error starting recording: {}", e);
                notify_error(&app_clone, "Error", &format!("{}", e));
                reset_state(&app_clone).await;
            }
        }
    });
}

/// Start voice recording
async fn start_recording(app: AppHandle) -> Result<()> {
    println!("[ISI] start_recording called");
    let state = app.state::<Arc<AppState>>();

    // Check if there's an image in clipboard first
    println!("[ISI] Checking clipboard for image...");
    if !has_image_in_clipboard() {
        println!("[ISI] No image in clipboard");
        notify(&app, "No Image", "Copy an image to the clipboard first");
        return Ok(());
    }
    println!("[ISI] Image found in clipboard");

    // Check if API keys are configured
    {
        let config = state.config.lock().await;
        println!("[ISI] Checking config - deepgram_key: {}, gemini_key: {}",
            config.deepgram_api_key.is_some(),
            config.gemini_api_key.is_some());
        if !config.is_configured() {
            println!("[ISI] API keys not configured");
            notify(
                &app,
                "Not Configured",
                "Click the tray icon → Open Config to add your API keys",
            );
            return Ok(());
        }
        println!("[ISI] API keys configured");
    }

    // Update state
    state.is_recording.store(true, Ordering::SeqCst);
    *state.recording_state.lock().await = RecordingState::Recording;
    update_tray_icon(&app, RecordingState::Recording);
    println!("[ISI] State updated to Recording");

    // Create and start recorder
    println!("[ISI] Creating AudioRecorder...");
    let mut recorder = AudioRecorder::new()
        .context("Microphone unavailable")?;
    println!("[ISI] Starting recording...");
    recorder.start_recording()
        .context("Could not start recording")?;
    *state.recorder.lock().await = Some(recorder);
    println!("[ISI] Recording started successfully");

    // Update UI
    update_tray_status(&app, "Recording... (press shortcut to stop)");
    notify(&app, "Recording", "Speak your instruction...");

    Ok(())
}

/// Stop recording and process the transformation
async fn stop_recording_and_process(app: AppHandle) -> Result<()> {
    println!("[ISI] stop_recording_and_process called");
    let state = app.state::<Arc<AppState>>();

    // Update state
    state.is_recording.store(false, Ordering::SeqCst);
    *state.recording_state.lock().await = RecordingState::Transcribing;
    update_tray_icon(&app, RecordingState::Transcribing);
    update_tray_status(&app, "Transcribing...");

    // Stop recorder and get recording data
    println!("[ISI] Stopping recorder...");
    let recording_data = {
        let mut recorder_guard = state.recorder.lock().await;
        if let Some(mut recorder) = recorder_guard.take() {
            recorder.stop_recording()?
        } else {
            eprintln!("[ISI] No active recording found!");
            anyhow::bail!("No active recording");
        }
    };
    println!("[ISI] Recorder stopped");

    // Get API keys
    let (deepgram_key, gemini_key) = {
        let config = state.config.lock().await;
        (
            config.deepgram_key()?.to_string(),
            config.gemini_key()?.to_string(),
        )
    };
    println!("[ISI] API keys retrieved");

    // Transcribe with Deepgram via whis-core
    notify(&app, "Processing", "Transcribing your voice...");

    let samples = recording_data.finalize_raw();
    println!("[ISI] Audio samples: {} samples", samples.len());

    // Create chunker for progressive transcription
    let (chunk_tx, chunk_rx) = tokio::sync::mpsc::unbounded_channel();

    // Send all samples as a single chunk
    use whis_core::ProgressiveChunk;
    let _ = chunk_tx.send(ProgressiveChunk {
        samples: samples.clone(),
        index: 0,
        has_leading_overlap: false,
    });
    drop(chunk_tx);

    println!("[ISI] Calling Deepgram for transcription...");
    let transcription = progressive_transcribe_cloud(
        &TranscriptionProvider::Deepgram,
        &deepgram_key,
        None, // language auto-detect
        chunk_rx,
        None, // no progress callback
    )
    .await
    .context("Transcription failed - check your internet connection")?;
    println!("[ISI] Transcription result: '{}'", transcription);

    if transcription.trim().is_empty() {
        println!("[ISI] Empty transcription - aborting");
        notify(&app, "Error", "Could not transcribe audio. Please try again.");
        *state.recording_state.lock().await = RecordingState::Idle;
        update_tray_icon(&app, RecordingState::Idle);
        update_tray_status(&app, "ISI Voice Image - Ready");
        return Ok(());
    }

    // Transform image
    *state.recording_state.lock().await = RecordingState::Transforming;
    update_tray_icon(&app, RecordingState::Transforming);
    update_tray_status(&app, "Transforming image...");
    notify(&app, "Transforming", &format!("\"{}\"", transcription.trim()));

    // Read image from clipboard
    println!("[ISI] Reading image from clipboard...");
    let original_image = read_image_from_clipboard()
        .context("Could not read clipboard")?;
    println!("[ISI] Image read: {}x{} ({} bytes)", original_image.width, original_image.height, original_image.rgba_data.len());

    // Transform with Gemini
    println!("[ISI] Creating Gemini client...");
    let gemini = GeminiClient::new(gemini_key)?;
    println!("[ISI] Calling Gemini to transform image...");
    let transformed_image = gemini
        .transform_image(&original_image, &transcription)
        .await
        .context("Image transformation failed")?;
    println!("[ISI] Transformed image: {}x{} ({} bytes)", transformed_image.width, transformed_image.height, transformed_image.rgba_data.len());

    // Write back to clipboard
    println!("[ISI] Writing transformed image to clipboard...");
    write_image_to_clipboard(&transformed_image)
        .context("Could not save to clipboard")?;
    println!("[ISI] Image written to clipboard");

    *state.recording_state.lock().await = RecordingState::Idle;
    update_tray_icon(&app, RecordingState::Idle);
    update_tray_status(&app, "ISI Voice Image - Ready");
    notify(&app, "Done!", "Transformed image is in your clipboard");
    println!("[ISI] Processing complete!");

    Ok(())
}

/// Process image transformation with a text command (no microphone needed)
/// This is useful for testing on MacinCloud or other environments without audio input
pub async fn process_with_text(app: AppHandle, command: &str) -> Result<()> {
    println!("[ISI] process_with_text called with command: '{}'", command);
    let state = app.state::<Arc<AppState>>();

    // Check if there's an image in clipboard
    println!("[ISI] Checking clipboard for image...");
    if !has_image_in_clipboard() {
        println!("[ISI] No image in clipboard");
        notify(&app, "No Image", "Copy an image to the clipboard first");
        return Ok(());
    }
    println!("[ISI] Image found in clipboard");

    // Check if Gemini API key is configured
    let gemini_key = {
        let config = state.config.lock().await;
        println!("[ISI] Checking config - gemini_key: {}", config.gemini_api_key.is_some());
        match &config.gemini_api_key {
            Some(key) => key.clone(),
            None => {
                println!("[ISI] Gemini API key not configured");
                notify(
                    &app,
                    "Not Configured",
                    "Click the tray icon → Open Config to add your Gemini API key",
                );
                return Ok(());
            }
        }
    };
    println!("[ISI] Gemini API key retrieved");

    // Update state to transforming
    *state.recording_state.lock().await = RecordingState::Transforming;
    update_tray_icon(&app, RecordingState::Transforming);
    update_tray_status(&app, "Transforming image...");
    notify(&app, "Transforming", &format!("\"{}\"", command));

    // Read image from clipboard
    println!("[ISI] Reading image from clipboard...");
    let original_image = read_image_from_clipboard()
        .context("Could not read clipboard")?;
    println!("[ISI] Image read: {}x{} ({} bytes)", original_image.width, original_image.height, original_image.rgba_data.len());

    // Transform with Gemini
    println!("[ISI] Creating Gemini client...");
    let gemini = GeminiClient::new(gemini_key)?;
    println!("[ISI] Calling Gemini to transform image...");
    let transformed_image = gemini
        .transform_image(&original_image, command)
        .await
        .context("Image transformation failed")?;
    println!("[ISI] Transformed image: {}x{} ({} bytes)", transformed_image.width, transformed_image.height, transformed_image.rgba_data.len());

    // Write back to clipboard
    println!("[ISI] Writing transformed image to clipboard...");
    write_image_to_clipboard(&transformed_image)
        .context("Could not save to clipboard")?;
    println!("[ISI] Image written to clipboard");

    *state.recording_state.lock().await = RecordingState::Idle;
    update_tray_icon(&app, RecordingState::Idle);
    update_tray_status(&app, "ISI Voice Image - Ready");
    notify(&app, "Done!", "Transformed image is in your clipboard");
    println!("[ISI] Processing complete!");

    Ok(())
}
