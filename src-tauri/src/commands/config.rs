use crate::services::config::ConfigService;
use crate::utils::config::{AppConfig, Config, ShortcutsConfig};
use crate::utils::error::AppError;

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> Result<Config, AppError> {
    ConfigService::get(&app)
}

#[tauri::command]
pub fn update_app_config(app: tauri::AppHandle, updates: AppConfig) -> Result<Config, AppError> {
    ConfigService::update_app(&app, updates)
}

#[tauri::command]
pub fn update_shortcuts_config(app: tauri::AppHandle, updates: ShortcutsConfig) -> Result<Config, AppError> {
    ConfigService::update_shortcuts(&app, updates)
}

#[tauri::command]
pub fn set_workspace(app: tauri::AppHandle, path: String) -> Result<Config, AppError> {
    ConfigService::set_workspace(&app, path)
}

#[tauri::command]
pub fn reset_config(app: tauri::AppHandle) -> Result<Config, AppError> {
    ConfigService::reset(&app)
}
