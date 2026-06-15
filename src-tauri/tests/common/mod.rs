use pentip_lib::db::queries::run_migrations;
use sqlx::SqlitePool;

/// Create an in-memory SQLite database with migrations applied.
pub async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");
    run_migrations(&pool).await.expect("Failed to run migrations");
    pool
}