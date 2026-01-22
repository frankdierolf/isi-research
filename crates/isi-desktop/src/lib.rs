//! ISI Desktop - Voice-controlled image manipulation
//!
//! A macOS application that transforms clipboard images using voice commands.

pub mod clipboard;
pub mod config;
pub mod gemini;
pub mod recording;

use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use config::Config;
use recording::{AppState, toggle_recording};

/// DTO for config serialization to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub deepgram_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub shortcut: String,
    pub mode: String,
    pub manual_prompt: String,
}

/// Setup the system tray
fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::image::Image;

    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&settings_item, &quit])?;

    let icon = Image::from_bytes(include_bytes!("../icons/tray-idle.png"))?;

    TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .tooltip("ISI Voice Image - Ready")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "settings" => {
                // Open settings window
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

/// Setup global keyboard shortcut
fn setup_shortcut(app: &tauri::App, shortcut_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();

    let shortcut = Shortcut::from_str(shortcut_str)
        .map_err(|e| format!("Invalid shortcut '{}': {}", shortcut_str, e))?;

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    let handle = app_handle.clone();
                    toggle_recording(handle);
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(shortcut)?;
    println!("Global shortcut registered: {}", shortcut_str);

    Ok(())
}

/// Tauri command to get current state
#[tauri::command]
fn get_status(state: tauri::State<Arc<AppState>>) -> String {
    if state.is_recording() {
        "recording".to_string()
    } else {
        "idle".to_string()
    }
}

/// Tauri command to check if configured
#[tauri::command]
async fn is_configured(state: tauri::State<'_, Arc<AppState>>) -> Result<bool, String> {
    let config = state.config.lock().await;
    Ok(config.is_configured())
}

/// Tauri command to get current config
#[tauri::command]
async fn get_config(state: tauri::State<'_, Arc<AppState>>) -> Result<ConfigDto, String> {
    let config = state.config.lock().await;
    Ok(ConfigDto {
        deepgram_api_key: config.deepgram_api_key.clone(),
        gemini_api_key: config.gemini_api_key.clone(),
        shortcut: config.shortcut.clone(),
        mode: config.mode.clone(),
        manual_prompt: config.manual_prompt.clone(),
    })
}

/// Tauri command to save config
#[tauri::command]
async fn save_config(
    state: tauri::State<'_, Arc<AppState>>,
    config_dto: ConfigDto,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.deepgram_api_key = config_dto.deepgram_api_key;
    config.gemini_api_key = config_dto.gemini_api_key;
    config.shortcut = config_dto.shortcut;
    config.mode = config_dto.mode;
    config.manual_prompt = config_dto.manual_prompt;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}

/// Main entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load config
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}. Using defaults.", e);
        Config::default()
    });

    let shortcut = config.shortcut.clone();
    let app_state = Arc::new(AppState::new(config));

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .setup(move |app| {
            // Setup system tray
            setup_tray(app)?;

            // Setup global shortcut
            if let Err(e) = setup_shortcut(app, &shortcut) {
                eprintln!("Failed to setup shortcut: {}", e);
            }

            // Request notification permission and show initial notification
            use tauri_plugin_notification::NotificationExt;
            use tauri_plugin_notification::PermissionState;

            let notification = app.notification();

            // Check permission state and request if needed
            match notification.permission_state() {
                Ok(PermissionState::Granted) => {
                    // Permission already granted, show welcome notification
                    let result = notification
                        .builder()
                        .title("ISI Voice Image")
                        .body(format!("Ready! Press {} to transform images.", shortcut))
                        .show();
                    if let Err(e) = result {
                        eprintln!("[ISI] Failed to show welcome notification: {}", e);
                    }
                }
                Ok(PermissionState::Denied) => {
                    eprintln!("[ISI] Notification permission denied by user");
                }
                Ok(PermissionState::Prompt) | Ok(PermissionState::PromptWithRationale) | Err(_) => {
                    // Request permission - this will prompt the user on macOS
                    match notification.request_permission() {
                        Ok(PermissionState::Granted) => {
                            let result = notification
                                .builder()
                                .title("ISI Voice Image")
                                .body(format!("Ready! Press {} to transform images.", shortcut))
                                .show();
                            if let Err(e) = result {
                                eprintln!("[ISI] Failed to show welcome notification: {}", e);
                            }
                        }
                        _ => {
                            eprintln!("[ISI] Notification permission not granted");
                        }
                    }
                }
            }

            // Handle dock icon click on macOS (reopen event)
            #[cfg(target_os = "macos")]
            {
                use tauri::Listener;
                let handle = app.handle().clone();
                app.listen("tauri://reopen", move |_| {
                    if let Some(window) = handle.get_webview_window("settings") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                });
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide window instead of closing (minimize to tray)
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![get_status, is_configured, get_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
