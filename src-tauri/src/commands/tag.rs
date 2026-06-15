use crate::db::TagRow;
use crate::services::tag::TagService;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn create_tag(name: String) -> Result<TagRow, AppError> {
    TagService::create(name).await
}

#[tauri::command]
pub async fn list_tags() -> Result<Vec<TagRow>, AppError> {
    TagService::list().await
}

#[tauri::command]
pub async fn delete_tag(id: String) -> Result<(), AppError> {
    TagService::delete(id).await
}

#[tauri::command]
pub async fn attach_tag(tag_id: String, target_type: String, target_id: String) -> Result<(), AppError> {
    TagService::attach(tag_id, target_type, target_id).await
}

#[tauri::command]
pub async fn detach_tag(tag_id: String, target_type: String, target_id: String) -> Result<(), AppError> {
    TagService::detach(tag_id, target_type, target_id).await
}

#[tauri::command]
pub async fn list_tags_for_target(target_type: String, target_id: String) -> Result<Vec<TagRow>, AppError> {
    TagService::list_for_target(target_type, target_id).await
}
