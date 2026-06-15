use crate::db::{FragmentRow, SearchResultRow};
use crate::services::fragment::FragmentService;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn create_fragment(content: String, note_type: String) -> Result<FragmentRow, AppError> {
    FragmentService::create(content, note_type).await
}

#[tauri::command]
pub async fn list_fragments() -> Result<Vec<FragmentRow>, AppError> {
    FragmentService::list().await
}

#[tauri::command]
pub async fn get_fragment(id: String) -> Result<FragmentRow, AppError> {
    FragmentService::get(&id).await
}

#[tauri::command]
pub async fn update_fragment(id: String, content: String) -> Result<FragmentRow, AppError> {
    FragmentService::update(id, content).await
}

#[tauri::command]
pub async fn delete_fragment(id: String) -> Result<(), AppError> {
    FragmentService::delete(id).await
}

#[tauri::command]
pub async fn search_fragments(query: String) -> Result<Vec<SearchResultRow>, AppError> {
    FragmentService::search(query).await
}