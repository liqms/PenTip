use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Row, SqlitePool};
use std::sync::OnceLock;

// 鈹€鈹€鈹€ DTOs 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentRow {
    pub id: String,
    pub content: String,
    #[serde(rename = "type")]
    pub note_type: String,
    pub source: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRow {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub project_type: String,
    pub status: String,
    pub metadata: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRow {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: Option<String>,
    #[serde(rename = "order")]
    pub sort_order: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultRow {
    pub id: String,
    pub content: String,
}

// 鈹€鈹€鈹€ Database Initialization 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

static POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Get or initialize the database pool.
pub async fn get_db(db_url: &str) -> Result<&'static SqlitePool, AppError> {
    if let Some(pool) = POOL.get() {
        return Ok(pool);
    }

    // Use SqliteConnectOptions with direct filename to avoid URL parsing issues on Windows
    let opts = SqliteConnectOptions::new()
        .filename(db_url)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts)
        .await
        .map_err(AppError::db_init)?;

    // Run migrations
    run_migrations(&pool).await?;

    POOL.set(pool).map_err(|_| AppError::pool_init())?;
    Ok(POOL.get().unwrap())
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    let sql = include_str!("../../sql/001_create_tables.sql");
    // Split by semicolons and execute each statement
    for statement in sql.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sqlx::query(trimmed)
                .execute(pool)
                .await
                .map_err(|e| AppError::db_query(format!("Migration error: {e} | SQL: {trimmed}")))?;
        }
    }
    Ok(())
}

// 鈹€鈹€鈹€ Helpers 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

fn fragment_from_row(row: &sqlx::sqlite::SqliteRow) -> FragmentRow {
    FragmentRow {
        id: row.get("id"),
        content: row.get("content"),
        note_type: row.get("type"),
        source: row.get("source"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

fn tag_from_row(row: &sqlx::sqlite::SqliteRow) -> TagRow {
    TagRow {
        id: row.get("id"),
        name: row.get("name"),
        created_at: row.get("created_at"),
    }
}

fn project_from_row(row: &sqlx::sqlite::SqliteRow) -> ProjectRow {
    ProjectRow {
        id: row.get("id"),
        title: row.get("title"),
        description: row.get("description"),
        project_type: row.get("type"),
        status: row.get("status"),
        metadata: row.get("metadata"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

fn page_from_row(row: &sqlx::sqlite::SqliteRow) -> PageRow {
    PageRow {
        id: row.get("id"),
        project_id: row.get("project_id"),
        title: row.get("title"),
        content: row.get("content"),
        sort_order: row.get("order"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

// 鈹€鈹€鈹€ Fragment queries 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn list_fragments(db: &SqlitePool) -> Result<Vec<FragmentRow>, AppError> {
    let rows = sqlx::query(
        "SELECT * FROM fragments WHERE deleted_at IS NULL ORDER BY created_at DESC"
    )
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(fragment_from_row).collect())
}

pub async fn get_fragment(db: &SqlitePool, id: &str) -> Result<FragmentRow, AppError> {
    sqlx::query("SELECT * FROM fragments WHERE id = ?1 AND deleted_at IS NULL")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(AppError::db_query)?
        .map(|row| fragment_from_row(&row))
        .ok_or_else(|| AppError::fragment_not_found(id))
}

pub async fn create_fragment(
    db: &SqlitePool,
    id: &str,
    content: &str,
    note_type: &str,
    source: Option<&str>,
    created_at: i64,
    updated_at: i64,
) -> Result<FragmentRow, AppError> {
    sqlx::query(
        "INSERT INTO fragments (id, content, type, source, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(id)
        .bind(content)
        .bind(note_type)
        .bind(source)
        .bind(created_at)
        .bind(updated_at)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    get_fragment(db, id).await
}

pub async fn update_fragment(
    db: &SqlitePool,
    id: &str,
    content: &str,
    updated_at: i64,
) -> Result<FragmentRow, AppError> {
    sqlx::query(
        "UPDATE fragments SET content = ?1, updated_at = ?2 WHERE id = ?3 AND deleted_at IS NULL"
    )
        .bind(content)
        .bind(updated_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    get_fragment(db, id).await
}

pub async fn delete_fragment(
    db: &SqlitePool,
    id: &str,
    deleted_at: i64,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE fragments SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL"
    )
        .bind(deleted_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}

pub async fn search_fragments(
    db: &SqlitePool,
    query: &str,
) -> Result<Vec<SearchResultRow>, AppError> {
    let rows = sqlx::query(
        "SELECT f.id, f.content FROM fragments_fts fts JOIN fragments f ON fts.rowid = f.rowid WHERE fragments_fts MATCH ?1 AND f.deleted_at IS NULL LIMIT 50"
    )
        .bind(query)
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(|row| SearchResultRow {
        id: row.get("id"),
        content: row.get("content"),
    }).collect())
}

// 鈹€鈹€鈹€ Tag queries 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn create_tag(
    db: &SqlitePool,
    id: &str,
    name: &str,
    created_at: i64,
) -> Result<TagRow, AppError> {
    sqlx::query("INSERT INTO tags (id, name, created_at) VALUES (?1, ?2, ?3)")
        .bind(id)
        .bind(name)
        .bind(created_at)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    let row = sqlx::query("SELECT * FROM tags WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(tag_from_row(&row))
}

pub async fn list_tags(db: &SqlitePool) -> Result<Vec<TagRow>, AppError> {
    let rows = sqlx::query("SELECT * FROM tags ORDER BY name ASC")
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(tag_from_row).collect())
}

pub async fn delete_tag(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM tag_relations WHERE tag_id = ?1")
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    sqlx::query("DELETE FROM tags WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}

pub async fn attach_tag(
    db: &SqlitePool,
    tag_id: &str,
    target_type: &str,
    target_id: &str,
    created_at: i64,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT OR IGNORE INTO tag_relations (tag_id, target_type, target_id, created_at) VALUES (?1, ?2, ?3, ?4)"
    )
        .bind(tag_id)
        .bind(target_type)
        .bind(target_id)
        .bind(created_at)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}

pub async fn detach_tag(
    db: &SqlitePool,
    tag_id: &str,
    target_type: &str,
    target_id: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "DELETE FROM tag_relations WHERE tag_id = ?1 AND target_type = ?2 AND target_id = ?3"
    )
        .bind(tag_id)
        .bind(target_type)
        .bind(target_id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}

pub async fn list_tags_for_target(
    db: &SqlitePool,
    target_type: &str,
    target_id: &str,
) -> Result<Vec<TagRow>, AppError> {
    let rows = sqlx::query(
        "SELECT t.* FROM tags t JOIN tag_relations tr ON t.id = tr.tag_id WHERE tr.target_type = ?1 AND tr.target_id = ?2 ORDER BY t.name ASC"
    )
        .bind(target_type)
        .bind(target_id)
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(tag_from_row).collect())
}

// 鈹€鈹€鈹€ Project queries 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn list_projects(db: &SqlitePool) -> Result<Vec<ProjectRow>, AppError> {
    let rows = sqlx::query(
        "SELECT * FROM projects WHERE deleted_at IS NULL ORDER BY updated_at DESC"
    )
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(project_from_row).collect())
}

pub async fn create_project(
    db: &SqlitePool,
    id: &str,
    title: &str,
    project_type: &str,
    created_at: i64,
    updated_at: i64,
) -> Result<ProjectRow, AppError> {
    sqlx::query(
        "INSERT INTO projects (id, title, type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)"
    )
        .bind(id)
        .bind(title)
        .bind(project_type)
        .bind(created_at)
        .bind(updated_at)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    let row = sqlx::query("SELECT * FROM projects WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(project_from_row(&row))
}

pub async fn update_project(
    db: &SqlitePool,
    id: &str,
    title: &str,
    updated_at: i64,
) -> Result<ProjectRow, AppError> {
    sqlx::query(
        "UPDATE projects SET title = ?1, updated_at = ?2 WHERE id = ?3 AND deleted_at IS NULL"
    )
        .bind(title)
        .bind(updated_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    let row = sqlx::query("SELECT * FROM projects WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(project_from_row(&row))
}

pub async fn delete_project(
    db: &SqlitePool,
    id: &str,
    deleted_at: i64,
) -> Result<(), AppError> {
    sqlx::query("UPDATE projects SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL")
        .bind(deleted_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}

// 鈹€鈹€鈹€ Page queries 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn list_pages(
    db: &SqlitePool,
    project_id: &str,
) -> Result<Vec<PageRow>, AppError> {
    let rows = sqlx::query(
        "SELECT * FROM pages WHERE project_id = ?1 AND deleted_at IS NULL ORDER BY \"order\" ASC"
    )
        .bind(project_id)
        .fetch_all(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(rows.iter().map(page_from_row).collect())
}

pub async fn create_page(
    db: &SqlitePool,
    id: &str,
    project_id: &str,
    title: &str,
    order: i64,
    created_at: i64,
    updated_at: i64,
) -> Result<PageRow, AppError> {
    sqlx::query(
        "INSERT INTO pages (id, project_id, title, content, \"order\", created_at, updated_at) VALUES (?1, ?2, ?3, '', ?4, ?5, ?6)"
    )
        .bind(id)
        .bind(project_id)
        .bind(title)
        .bind(order)
        .bind(created_at)
        .bind(updated_at)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    let row = sqlx::query("SELECT * FROM pages WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(page_from_row(&row))
}

pub async fn update_page(
    db: &SqlitePool,
    id: &str,
    title: &str,
    content: &str,
    updated_at: i64,
) -> Result<PageRow, AppError> {
    sqlx::query(
        "UPDATE pages SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4 AND deleted_at IS NULL"
    )
        .bind(title)
        .bind(content)
        .bind(updated_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    let row = sqlx::query("SELECT * FROM pages WHERE id = ?1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(page_from_row(&row))
}

pub async fn delete_page(
    db: &SqlitePool,
    id: &str,
    deleted_at: i64,
) -> Result<(), AppError> {
    sqlx::query("UPDATE pages SET deleted_at = ?1 WHERE id = ?2 AND deleted_at IS NULL")
        .bind(deleted_at)
        .bind(id)
        .execute(db)
        .await
        .map_err(AppError::db_query)?;
    Ok(())
}
