use crate::db::{Database, ProjectRow};
use crate::utils::error::AppError;

/// Project service — encapsulates business logic for project operations.
pub struct ProjectService;

impl ProjectService {
    pub async fn create(title: String, project_type: String) -> Result<ProjectRow, AppError> {
        log::info!("Creating project: {title} ({project_type})");
        let row = Database::create_project(&title, &project_type).await?;
        log::info!("Project created: id={}", row.id);
        Ok(row)
    }

    pub async fn list() -> Result<Vec<ProjectRow>, AppError> {
        log::debug!("Listing projects");
        Database::list_projects().await
    }

    pub async fn update(id: String, title: String) -> Result<ProjectRow, AppError> {
        log::info!("Updating project: {id}");
        let row = Database::update_project(&id, &title).await?;
        log::info!("Project updated: {id}");
        Ok(row)
    }

    pub async fn delete(id: String) -> Result<(), AppError> {
        log::info!("Deleting project: {id}");
        Database::delete_project(&id).await?;
        log::info!("Project deleted: {id}");
        Ok(())
    }
}