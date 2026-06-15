use crate::db::ProjectRow;
use crate::services::project::ProjectService;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn create_project(title: String, project_type: String) -> Result<ProjectRow, AppError> {
    ProjectService::create(title, project_type).await
}

#[tauri::command]
pub async fn list_projects() -> Result<Vec<ProjectRow>, AppError> {
    ProjectService::list().await
}

#[tauri::command]
pub async fn update_project(id: String, title: String) -> Result<ProjectRow, AppError> {
    ProjectService::update(id, title).await
}

#[tauri::command]
pub async fn delete_project(id: String) -> Result<(), AppError> {
    ProjectService::delete(id).await
}
