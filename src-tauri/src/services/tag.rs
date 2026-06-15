use crate::db::{Database, TagRow};
use crate::utils::error::AppError;

/// Tag service — encapsulates business logic for tag operations.
pub struct TagService;

impl TagService {
    pub async fn create(name: String) -> Result<TagRow, AppError> {
        log::info!("Creating tag: {name}");
        let row = Database::create_tag(&name).await?;
        log::info!("Tag created: id={}", row.id);
        Ok(row)
    }

    pub async fn list() -> Result<Vec<TagRow>, AppError> {
        log::debug!("Listing tags");
        Database::list_tags().await
    }

    pub async fn delete(id: String) -> Result<(), AppError> {
        log::info!("Deleting tag: {id}");
        Database::delete_tag(&id).await?;
        log::info!("Tag deleted: {id}");
        Ok(())
    }

    pub async fn attach(tag_id: String, target_type: String, target_id: String) -> Result<(), AppError> {
        log::info!("Attaching tag {tag_id} to {target_type}/{target_id}");
        Database::attach_tag(&tag_id, &target_type, &target_id).await
    }

    pub async fn detach(tag_id: String, target_type: String, target_id: String) -> Result<(), AppError> {
        log::info!("Detaching tag {tag_id} from {target_type}/{target_id}");
        Database::detach_tag(&tag_id, &target_type, &target_id).await
    }

    pub async fn list_for_target(target_type: String, target_id: String) -> Result<Vec<TagRow>, AppError> {
        log::debug!("Listing tags for {target_type}/{target_id}");
        Database::list_tags_for_target(&target_type, &target_id).await
    }
}