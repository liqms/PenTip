use tauri::Manager;
use crate::utils::config::{AppConfig, Config, ShortcutsConfig};
use crate::utils::error::AppError;

/// Config service — business logic for reading/updating app configuration.
pub struct ConfigService;

impl ConfigService {
    /// Return the current `Config` from Tauri state.
    pub fn get(app: &tauri::AppHandle) -> Result<Config, AppError> {
        log::debug!("ConfigService::get");
        let state: tauri::State<'_, Config> = app.state::<Config>();
        Ok(state.inner().clone())
    }

    /// Partially update `AppConfig` fields and persist to disk.
    /// Merges partial `updates` on top of the current config.
    pub fn update_app(app: &tauri::AppHandle, updates: AppConfig) -> Result<Config, AppError> {
        log::info!("ConfigService::update_app");

        let mut state = app.state::<Config>().inner().clone();
        state.app = updates;
        state.save(app)?;

        log::info!("App config saved");
        Ok(state)
    }

    /// Partially update `ShortcutsConfig` fields and persist to disk.
    pub fn update_shortcuts(app: &tauri::AppHandle, updates: ShortcutsConfig) -> Result<Config, AppError> {
        log::info!("ConfigService::update_shortcuts");

        let mut state = app.state::<Config>().inner().clone();
        state.shortcuts = updates;
        state.save(app)?;

        log::info!("Shortcuts config saved");
        Ok(state)
    }

    /// Update the workspace path and persist the change.
    pub fn set_workspace(app: &tauri::AppHandle, path: String) -> Result<Config, AppError> {
        log::info!("ConfigService::set_workspace: {path}");

        let cfg_dir = Config::cfg_dir(app)?;
        std::fs::create_dir_all(&path).map_err(AppError::config_update)?;
        std::fs::write(cfg_dir.join(".workspace"), &path)
            .map_err(AppError::config_update)?;

        let mut state = app.state::<Config>().inner().clone();
        state.workspace = path;
        state.save(app)?;

        log::info!("Workspace updated");
        Ok(state)
    }

    /// Reset config to factory defaults by deleting config files and recreating them.
    pub fn reset(app: &tauri::AppHandle) -> Result<Config, AppError> {
        log::info!("ConfigService::reset");

        let cfg_dir = Config::cfg_dir(app)?;

        // Remove existing config files
        for file in &["app.json", "shortcuts.json"] {
            let p = cfg_dir.join(file);
            if p.exists() {
                std::fs::remove_file(&p).map_err(AppError::config_update)?;
            }
        }

        // Re-create with defaults, reusing the public resolve_workspace
        let config = Config {
            app: AppConfig::default(),
            shortcuts: ShortcutsConfig::default(),
            workspace: Config::resolve_workspace_handle(app)?,
        };

        config.save(app)?;
        log::info!("Config reset to defaults");
        Ok(config)
    }
}

