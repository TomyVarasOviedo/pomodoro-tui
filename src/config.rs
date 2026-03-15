use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_focus")]
    pub focus_minutes:u64,

    #[serde(default = "default_short_break")]
    pub short_break_minutes:u64,

    #[serde(default = "default_long_break")]
    pub long_break_minutes:u64,

    #[serde(default = "default_long_break_interval")]
    pub long_break_interval: u32,

    #[serde(default)]
    pub notifications:NotificationConfig,
    
}
impl Default for Config {
    fn default() -> Self {
        Self { 
            focus_minutes: default_focus(), 
            short_break_minutes: default_short_break(), 
            long_break_minutes: default_long_break(),
            long_break_interval: default_long_break_interval(), 
            notifications: NotificationConfig::default(),
        }
    }
}
impl Config {
    pub fn config_path() -> PathBuf{
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("pomors")
            .join("config.toml")
    }

    pub fn load() -> Self{
        let path = Self::config_path();
        if path.exists() {
            let raw = std::fs::read_to_string(&path).unwrap_or_default();
            match toml::from_str::<Config>(&raw) {
                Ok(cfg) => return cfg,
                Err(e) => eprintln!("Config Error: ({e}) "),
            }
        }

        let default = Config::default();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(serialized) = toml::to_string_pretty(&default) {
            let _ = std::fs::write(&path, serialized);
            eprintln!("✓ Created default config at {}", path.display());
        }
        default
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationConfig {

    #[serde(default = "bool_true")]
    pub enabled:bool,

    #[serde(default = "default_urgency")]
    pub urgency: String,

    #[serde(default = "default_icon")]
    pub icon: String,
 
    #[serde(default)]
    pub bell: bool,
}
impl Default for NotificationConfig {
    fn default() -> Self {
        Self { 
            enabled: true, 
            urgency: default_urgency(), 
            icon: default_icon(), 
            bell: false, 
        }
    }
}


// Valores por defecto
fn default_focus() -> u64 { 30 }
fn default_short_break() -> u64 { 5 }
fn default_long_break() -> u64 { 15 }
fn default_long_break_interval() -> u32 { 5 }
fn bool_true() -> bool { true }
fn default_urgency() -> String { "normal".into() }
fn default_icon() -> String { "appointment-soon".into() }