//! Configuration management for ISI Desktop
//!
//! Stores settings in:
//! - macOS: ~/Library/Application Support/isi-research/config.toml
//! - Linux: ~/.config/isi-research/config.toml

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Deepgram API key for voice transcription
    pub deepgram_api_key: Option<String>,
    /// Google Gemini API key for image transformation
    pub gemini_api_key: Option<String>,
    /// Global shortcut to trigger recording (default: Cmd+Shift+I)
    #[serde(default = "default_shortcut")]
    pub shortcut: String,
    /// Mode: "voice" for voice recording, "manual" for predefined prompt
    #[serde(default = "default_mode")]
    pub mode: String,
    /// Default prompt for manual mode
    #[serde(default = "default_manual_prompt")]
    pub manual_prompt: String,
}

fn default_shortcut() -> String {
    "Cmd+Shift+I".to_string()
}

fn default_mode() -> String {
    "voice".to_string()
}

fn default_manual_prompt() -> String {
    "make it black and white".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            deepgram_api_key: None,
            gemini_api_key: None,
            shortcut: default_shortcut(),
            mode: default_mode(),
            manual_prompt: default_manual_prompt(),
        }
    }
}

impl Config {
    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("isi-research");
        Ok(config_dir)
    }

    /// Get the config file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
            let config: Config = toml::from_str(&contents)
                .with_context(|| "Failed to parse config file")?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

        let config_path = Self::config_path()?;
        let contents = self.to_toml_with_comments();

        fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    /// Generate TOML with helpful comments
    fn to_toml_with_comments(&self) -> String {
        let mut output = String::new();

        output.push_str("# ISI Voice Image Configuration\n");
        output.push_str("# ============================\n\n");

        output.push_str("# Deepgram API Key (required for voice transcription)\n");
        output.push_str("# Get your key at: https://console.deepgram.com/\n");
        match &self.deepgram_api_key {
            Some(key) => output.push_str(&format!("deepgram_api_key = \"{}\"\n\n", key)),
            None => output.push_str("# deepgram_api_key = \"paste-your-deepgram-api-key-here\"\n\n"),
        }

        output.push_str("# Google Gemini API Key (required for image transformation)\n");
        output.push_str("# Get your key at: https://aistudio.google.com/apikey\n");
        match &self.gemini_api_key {
            Some(key) => output.push_str(&format!("gemini_api_key = \"{}\"\n\n", key)),
            None => output.push_str("# gemini_api_key = \"paste-your-gemini-api-key-here\"\n\n"),
        }

        output.push_str("# Keyboard shortcut to trigger voice recording\n");
        output.push_str("# Default: Cmd+Shift+I (macOS)\n");
        output.push_str(&format!("shortcut = \"{}\"\n\n", self.shortcut));

        output.push_str("# Mode: \"voice\" for voice recording, \"manual\" for predefined prompt\n");
        output.push_str(&format!("mode = \"{}\"\n\n", self.mode));

        output.push_str("# Default prompt for manual mode\n");
        output.push_str(&format!("manual_prompt = \"{}\"\n", self.manual_prompt));

        output
    }

    /// Check if API keys are configured based on mode
    pub fn is_configured(&self) -> bool {
        if self.mode == "manual" {
            // Manual mode only needs Gemini
            self.gemini_api_key.is_some()
        } else {
            // Voice mode needs both
            self.deepgram_api_key.is_some() && self.gemini_api_key.is_some()
        }
    }

    /// Check if in manual mode
    pub fn is_manual_mode(&self) -> bool {
        self.mode == "manual"
    }

    /// Get Deepgram API key or error
    pub fn deepgram_key(&self) -> Result<&str> {
        self.deepgram_api_key
            .as_deref()
            .context("Deepgram API key not configured. Use 'Open Config' from the menu")
    }

    /// Get Gemini API key or error
    pub fn gemini_key(&self) -> Result<&str> {
        self.gemini_api_key
            .as_deref()
            .context("Gemini API key not configured. Use 'Open Config' from the menu")
    }
}
