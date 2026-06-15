use crate::db::queries;
use crate::utils;
use crate::utils::error::AppError;
use std::sync::OnceLock;

static DB_URL: OnceLock<String> = OnceLock::new();

// Re-export DTOs for external consumers
pub use crate::db::queries::{
    FragmentRow, PageRow, ProjectRow, SearchResultRow, TagRow,
};

/// Unified database access layer.
///
/// All data operations go through this struct. It manages the
/// connection pool internally and handles ID generation, timestamps,
/// and soft-delete concerns so callers don't have to.
pub struct Database;

impl Database {
    // ─── Lifecycle ───────────────────────────

    /// Initialize the database pool and apply pending migrations.
    /// Safe to call multiple times — the pool is a singleton.
    pub async fn init(db_url: &str) -> Result<(), AppError> {
        let _ = DB_URL.set(db_url.to_string());
        queries::get_db(db_url).await?;
        Ok(())
    }

    // ─── Fragment ─────────────────────────────

    pub async fn create_fragment(content: &str, note_type: &str) -> Result<FragmentRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = utils::now_millis();
        queries::create_fragment(db, &id, content, note_type, None, now, now).await
    }

    pub async fn list_fragments() -> Result<Vec<FragmentRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::list_fragments(db).await
    }

    pub async fn get_fragment(id: &str) -> Result<FragmentRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::get_fragment(db, id).await
    }

    pub async fn update_fragment(id: &str, content: &str) -> Result<FragmentRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::update_fragment(db, id, content, now).await
    }

    pub async fn delete_fragment(id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::delete_fragment(db, id, now).await
    }

    pub async fn search_fragments(query: &str) -> Result<Vec<SearchResultRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::search_fragments(db, query).await
    }

    // ─── Tag ─────────────────────────────────

    pub async fn create_tag(name: &str) -> Result<TagRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = utils::now_millis();
        queries::create_tag(db, &id, name, now).await
    }

    pub async fn list_tags() -> Result<Vec<TagRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::list_tags(db).await
    }

    pub async fn delete_tag(id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::delete_tag(db, id).await
    }

    pub async fn attach_tag(tag_id: &str, target_type: &str, target_id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::attach_tag(db, tag_id, target_type, target_id, now).await
    }

    pub async fn detach_tag(tag_id: &str, target_type: &str, target_id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::detach_tag(db, tag_id, target_type, target_id).await
    }

    pub async fn list_tags_for_target(target_type: &str, target_id: &str) -> Result<Vec<TagRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::list_tags_for_target(db, target_type, target_id).await
    }

    // ─── Project ─────────────────────────────

    pub async fn create_project(title: &str, project_type: &str) -> Result<ProjectRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = utils::now_millis();
        queries::create_project(db, &id, title, project_type, now, now).await
    }

    pub async fn list_projects() -> Result<Vec<ProjectRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::list_projects(db).await
    }

    pub async fn update_project(id: &str, title: &str) -> Result<ProjectRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::update_project(db, id, title, now).await
    }

    pub async fn delete_project(id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::delete_project(db, id, now).await
    }

    // ─── Page ────────────────────────────────

    pub async fn create_page(project_id: &str, title: &str) -> Result<PageRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = utils::now_millis();
        queries::create_page(db, &id, project_id, title, 0, now, now).await
    }

    pub async fn list_pages(project_id: &str) -> Result<Vec<PageRow>, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        queries::list_pages(db, project_id).await
    }

    pub async fn update_page(id: &str, title: &str, content: &str) -> Result<PageRow, AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::update_page(db, id, title, content, now).await
    }

    pub async fn delete_page(id: &str) -> Result<(), AppError> {
        let db = queries::get_db(DB_URL.get().expect("Database not initialized")).await?;
        let now = utils::now_millis();
        queries::delete_page(db, id, now).await
    }
}