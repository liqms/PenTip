use crate::db::{Database, PageRow};
use crate::utils::error::AppError;

/// Page service — encapsulates business logic for page operations.
pub struct PageService;

impl PageService {
    pub async fn create(project_id: String, title: String) -> Result<PageRow, AppError> {
        log::info!("Creating page: {title} (project={project_id})");
        let row = Database::create_page(&project_id, &title).await?;
        log::info!("Page created: id={}", row.id);
        Ok(row)
    }

    pub async fn list(project_id: String) -> Result<Vec<PageRow>, AppError> {
        log::debug!("Listing pages for project: {project_id}");
        Database::list_pages(&project_id).await
    }

    pub async fn update(id: String, title: String, content: String) -> Result<PageRow, AppError> {
        log::info!("Updating page: {id}");
        let row = Database::update_page(&id, &title, &content).await?;
        log::info!("Page updated: {id}");
        Ok(row)
    }

    pub async fn delete(id: String) -> Result<(), AppError> {
        log::info!("Deleting page: {id}");
        Database::delete_page(&id).await?;
        log::info!("Page deleted: {id}");
        Ok(())
    }
}