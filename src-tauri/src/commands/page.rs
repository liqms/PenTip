use crate::db::PageRow;
use crate::services::page::PageService;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn create_page(project_id: String, title: String) -> Result<PageRow, AppError> {
    PageService::create(project_id, title).await
}

#[tauri::command]
pub async fn list_pages(project_id: String) -> Result<Vec<PageRow>, AppError> {
    PageService::list(project_id).await
}

#[tauri::command]
pub async fn update_page(id: String, title: String, content: String) -> Result<PageRow, AppError> {
    PageService::update(id, title, content).await
}

#[tauri::command]
pub async fn delete_page(id: String) -> Result<(), AppError> {
    PageService::delete(id).await
}
