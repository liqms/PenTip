use serde::{Deserialize, Serialize};


/// Application-wide configuration (app.json).
///
/// Reference: docs/architecture.md §5.4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default = "default_window")]
    pub window: WindowConfig,
    #[serde(default)]
    pub theme: ThemeMode,
    #[serde(default = "default_locale")]
    pub locale: String,
    /// Custom user data path, null means platform default.
    pub user_data_path: Option<String>,
    #[serde(default)]
    pub quick_capture: QuickCaptureConfig,
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub auto_save: AutoSaveConfig,
    #[serde(default)]
    pub backup: BackupConfig,
    #[serde(default)]
    pub search: SearchConfig,
    #[serde(default)]
    pub startup: StartupConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: default_window(),
            theme: ThemeMode::default(),
            locale: default_locale(),
            user_data_path: None,
            quick_capture: QuickCaptureConfig::default(),
            editor: EditorConfig::default(),
            auto_save: AutoSaveConfig::default(),
            backup: BackupConfig::default(),
            search: SearchConfig::default(),
            startup: StartupConfig::default(),
        }
    }
}

// ── Sub-configs ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub maximized: bool,
    pub fullscreen: bool,
}

fn default_window() -> WindowConfig {
    WindowConfig {
        width: 1200, height: 800,
        x: None, y: None,
        maximized: false, fullscreen: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum ThemeMode {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    #[default]
    System,
}


fn default_locale() -> String { "zh-CN".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickCaptureConfig {
    pub shortcut: String,
    pub auto_save: bool,
    pub stay_on_top: bool,
}

impl Default for QuickCaptureConfig {
    fn default() -> Self {
        Self {
            shortcut: "Ctrl+Shift+K".into(),
            auto_save: true,
            stay_on_top: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorConfig {
    pub font_size: u32,
    pub font_family: String,
    pub line_height: f64,
    pub show_line_number: bool,
    pub enable_spell_check: bool,
    pub tab_size: u32,
    pub toolbar: ToolbarMode,
    pub bubble_menu: bool,
    pub slash_command: bool,
    pub auto_format: bool,
    pub typewriter_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum ToolbarMode {
    #[serde(rename = "default")]
    #[default]
    Default,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "none")]
    None,
}


impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_family: String::new(),
            line_height: 1.7,
            show_line_number: false,
            enable_spell_check: false,
            tab_size: 2,
            toolbar: ToolbarMode::Default,
            bubble_menu: true,
            slash_command: true,
            auto_format: true,
            typewriter_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoSaveConfig {
    pub enabled: bool,
    pub interval_ms: u64,
}

impl Default for AutoSaveConfig {
    fn default() -> Self { Self { enabled: true, interval_ms: 10_000 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupConfig {
    pub enabled: bool,
    pub interval_hours: u32,
    pub max_count: u32,
    pub compress: bool,
}

impl Default for BackupConfig {
    fn default() -> Self { Self { enabled: true, interval_hours: 24, max_count: 30, compress: false } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchConfig {
    pub max_results: u32,
    pub include_deleted: bool,
}

impl Default for SearchConfig {
    fn default() -> Self { Self { max_results: 50, include_deleted: false } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupConfig {
    pub launch_on_boot: bool,
    pub restore_last_session: bool,
    pub minimize_to_tray: bool,
}

impl Default for StartupConfig {
    fn default() -> Self { Self { launch_on_boot: false, restore_last_session: true, minimize_to_tray: true } }
}

