use crate::db::{Database, FragmentRow, SearchResultRow};
use crate::utils::error::AppError;

/// Fragment service — encapsulates business logic for fragment operations.
pub struct FragmentService;

impl FragmentService {
    pub async fn create(content: String, note_type: String) -> Result<FragmentRow, AppError> {
        log::info!("Creating fragment: type={note_type}");
        let row = Database::create_fragment(&content, &note_type).await?;
        log::info!("Fragment created: id={}", row.id);
        Ok(row)
    }

    pub async fn list() -> Result<Vec<FragmentRow>, AppError> {
        log::debug!("Listing fragments");
        Database::list_fragments().await
    }

    pub async fn get(id: &str) -> Result<FragmentRow, AppError> {
        log::debug!("Getting fragment: {id}");
        Database::get_fragment(id).await
    }

    pub async fn update(id: String, content: String) -> Result<FragmentRow, AppError> {
        log::info!("Updating fragment: {id}");
        let row = Database::update_fragment(&id, &content).await?;
        log::info!("Fragment updated: {id}");
        Ok(row)
    }

    pub async fn delete(id: String) -> Result<(), AppError> {
        log::info!("Deleting fragment: {id}");
        Database::delete_fragment(&id).await?;
        log::info!("Fragment deleted: {id}");
        Ok(())
    }

    pub async fn search(query: String) -> Result<Vec<SearchResultRow>, AppError> {
        log::debug!("Searching fragments: query=\"{query}\"");
        Database::search_fragments(&query).await
    }
}