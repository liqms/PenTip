pub mod ai_providers;
pub mod app;
pub mod shortcuts;
pub use app::AppConfig;
pub use shortcuts::ShortcutsConfig;

use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

/// Top-level configuration manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: app::AppConfig,
    pub shortcuts: shortcuts::ShortcutsConfig,
    pub workspace: String,
}

impl Config {
    pub fn load(app: &tauri::App) -> Result<Self, AppError> {
        let cfg_dir = Self::ensure_cfg_dir(app)?;
        let workspace = Self::resolve_workspace(app, &cfg_dir)?;

        let app_cfg = Self::load_json::<app::AppConfig>(&cfg_dir.join("app.json"))?
            .unwrap_or_else(|| app::AppConfig {
                window: app::WindowConfig { width: 1200, height: 800, x: None, y: None, maximized: false, fullscreen: false },
                theme: app::ThemeMode::System,
                locale: "zh-CN".into(),
                user_data_path: None,
                quick_capture: app::QuickCaptureConfig { shortcut: "Ctrl+Shift+K".into(), auto_save: true, stay_on_top: true },
                editor: app::EditorConfig {
                    font_size: 16, font_family: String::new(), line_height: 1.7,
                    show_line_number: false, enable_spell_check: false, tab_size: 2,
                    toolbar: app::ToolbarMode::Default, bubble_menu: true,
                    slash_command: true, auto_format: true, typewriter_mode: false,
                },
                auto_save: app::AutoSaveConfig { enabled: true, interval_ms: 10_000 },
                backup: app::BackupConfig { enabled: true, interval_hours: 24, max_count: 30, compress: false },
                search: app::SearchConfig { max_results: 50, include_deleted: false },
                startup: app::StartupConfig { launch_on_boot: false, restore_last_session: true, minimize_to_tray: true },
            });

        let shortcuts_cfg = Self::load_json::<shortcuts::ShortcutsConfig>(&cfg_dir.join("shortcuts.json"))?
            .unwrap_or_else(Self::default_shortcuts);

        Ok(Self { app: app_cfg, shortcuts: shortcuts_cfg, workspace })
    }

    pub fn workspace_path(&self) -> PathBuf {
        PathBuf::from(&self.workspace)
    }

    /// Convert a file path to a sqlx-compatible SQLite URL.
    /// On Windows, backslashes are converted to forward slashes.
    fn format_sqlite_url(path: PathBuf) -> String {
        path.to_string_lossy().replace("\\", "/").to_string()
    }

    pub fn db_url(&self) -> String {
        Self::format_sqlite_url(self.workspace_path().join("pentip.db"))
    }

    // ── Persistence ────────────────────────────

    /// Return the config directory path (app config dir, e.g. APPDATA/PenTip).
    pub fn cfg_dir(app: &tauri::AppHandle) -> Result<PathBuf, AppError> {
        let dir = app.path().app_config_dir().map_err(AppError::config_load)?;
        Ok(dir)
    }

    /// Saves `app.json` and `shortcuts.json` to disk.
    pub fn save(&self, app: &tauri::AppHandle) -> Result<(), AppError> {
        let cfg_dir = Self::cfg_dir(app)?;
        Self::save_json(&cfg_dir.join("app.json"), &self.app)?;
        Self::save_json(&cfg_dir.join("shortcuts.json"), &self.shortcuts)?;
        Ok(())
    }

    /// Resolve the workspace directory from `.workspace` file or default to `~/Documents/PenTip`.
    fn resolve_workspace(app: &tauri::App, cfg_dir: &std::path::Path) -> Result<String, AppError> {
        let meta_path = cfg_dir.join(".workspace");
        let path = if meta_path.exists() {
            let val = std::fs::read_to_string(&meta_path).map_err(AppError::config_load)?;
            val.trim().to_string()
        } else {
            let home = app.path().home_dir().map_err(AppError::config_load)?;
            home.join("Documents").join("PenTip").to_string_lossy().to_string()
        };
        std::fs::create_dir_all(&path).map_err(AppError::config_load)?;
        Ok(path)
    }

    /// Public version of `resolve_workspace` that accepts `&AppHandle` for use by services.
    pub fn resolve_workspace_handle(app: &tauri::AppHandle) -> Result<String, AppError> {
        let cfg_dir = Self::cfg_dir(app)?;
        let meta_path = cfg_dir.join(".workspace");
        let path = if meta_path.exists() {
            let val = std::fs::read_to_string(&meta_path).map_err(AppError::config_load)?;
            val.trim().to_string()
        } else {
            let home = app.path().home_dir().map_err(AppError::config_load)?;
            home.join("Documents").join("PenTip").to_string_lossy().to_string()
        };
        std::fs::create_dir_all(&path).map_err(AppError::config_load)?;
        Ok(path)
    }

    fn save_json<T: serde::Serialize>(path: &std::path::Path, value: &T) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(value).map_err(AppError::config_save)?;
        std::fs::write(path, &json).map_err(AppError::config_save)?;
        Ok(())
    }

    fn ensure_cfg_dir(app: &tauri::App) -> Result<PathBuf, AppError> {
        let dir = app.path().app_config_dir().map_err(AppError::config_load)?;
        std::fs::create_dir_all(&dir).map_err(AppError::config_load)?;
        Ok(dir)
    }

    fn load_json<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Result<Option<T>, AppError> {
        if path.exists() {
            let content = std::fs::read_to_string(path).map_err(AppError::config_load)?;
            serde_json::from_str(&content).map(Some).map_err(AppError::config_load)
        } else {
            Ok(None)
        }
    }

    fn default_shortcuts() -> shortcuts::ShortcutsConfig {
        shortcuts::ShortcutsConfig {
            global: shortcuts::GlobalShortcuts {
                capture: "Ctrl+Shift+K".into(), capture_clipboard: "Ctrl+Shift+M".into(),
            },
            app: shortcuts::AppShortcuts {
                new_note: "Ctrl+N".into(), new_project: "Ctrl+Shift+N".into(),
                quick_search: "Ctrl+P".into(), toggle_sidebar: "Ctrl+B".into(),
                open_settings: "Ctrl+,".into(),
            },
            editor: shortcuts::EditorShortcuts {
                bold: "Ctrl+B".into(), italic: "Ctrl+I".into(), underline: "Ctrl+U".into(),
                strikethrough: "Ctrl+Shift+S".into(), code: "Ctrl+E".into(),
                link: "Ctrl+K".into(), save: "Ctrl+S".into(),
            },
        }
    }
}
