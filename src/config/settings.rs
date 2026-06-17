//! Configuration: load/save /opt/rec25/config/config.toml.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONFIG_PATH: &str = "/opt/rec25/config/config.toml";
pub const DEFAULT_OUTPUT_DIR: &str = "/opt/rec25/output";
pub const DEFAULT_LOG_DIR: &str = "/opt/rec25/logs";

/// Top-level config — serialised as TOML with [general] and [api_keys] sections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub api_keys: ApiKeys,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct General {
    /// Directory where all output files are saved.
    pub output_dir: String,
    /// Directory where the log file lives.
    pub log_dir: String,
    /// Per-command timeout in seconds (default: 300).
    pub timeout_secs: u64,
    /// Number of lines shown in the output preview box.
    pub preview_lines: usize,
    /// Enable ANSI colour output.
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    /// Shodan CLI API key.
    pub shodan: String,
    /// GitHub personal access token (for github-subdomains).
    pub github: String,
    /// Censys API ID.
    pub censys_id: String,
    /// Censys API secret.
    pub censys_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: General {
                output_dir: DEFAULT_OUTPUT_DIR.to_string(),
                log_dir: DEFAULT_LOG_DIR.to_string(),
                timeout_secs: 300,
                preview_lines: 10,
                color: true,
            },
            api_keys: ApiKeys {
                shodan: String::new(),
                github: String::new(),
                censys_id: String::new(),
                censys_secret: String::new(),
            },
        }
    }
}

impl Config {
    /// Load from the default path, creating defaults if the file is absent.
    pub fn load_or_create() -> Result<Self> {
        if Path::new(CONFIG_PATH).exists() {
            let raw = std::fs::read_to_string(CONFIG_PATH)
                .with_context(|| format!("Reading {CONFIG_PATH}"))?;
            toml::from_str(&raw).with_context(|| "Parsing config.toml")
        } else {
            let cfg = Config::default();
            cfg.save()?;
            Ok(cfg)
        }
    }

    /// Persist the current config to disk.
    pub fn save(&self) -> Result<()> {
        std::fs::create_dir_all(Path::new(CONFIG_PATH).parent().unwrap())?;
        let toml_str = toml::to_string_pretty(self).context("Serialising config")?;
        std::fs::write(CONFIG_PATH, toml_str).with_context(|| format!("Writing {CONFIG_PATH}"))
    }

    // ── Convenience accessors so callers can write config.output_dir ──────

    pub fn output_dir(&self) -> &str { &self.general.output_dir }
    pub fn log_dir(&self)    -> &str { &self.general.log_dir }
    pub fn timeout_secs(&self) -> u64 { self.general.timeout_secs }
    pub fn preview_lines(&self) -> usize { self.general.preview_lines }
}
