use crate::services::file::FileService;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn export_markdown(app: tauri::AppHandle, content: String, filename: String) -> Result<String, AppError> {
    FileService::export_markdown(&app, content, filename).await
}
