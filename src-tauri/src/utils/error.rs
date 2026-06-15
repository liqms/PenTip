use serde::Serialize;

/// Standard application error with a machine-readable code.
///
/// Tauri commands return `Result<T, AppError>`, and the frontend
/// receives a JSON object `{ code: string, message: string }`.
#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
}

impl AppError {
    // 鈹€鈹€ Database 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

    pub fn db_init(e: impl std::fmt::Display) -> Self {
        Self { code: "DB_INIT", message: format!("Database initialization failed: {e}") }
    }

    pub fn db_query(e: impl std::fmt::Display) -> Self {
        Self { code: "DB_QUERY", message: format!("Database query failed: {e}") }
    }

    pub fn fragment_not_found(id: &str) -> Self {
        Self { code: "FRAGMENT_NOT_FOUND", message: format!("Fragment not found: {id}") }
    }

    pub fn project_not_found(id: &str) -> Self {
        Self { code: "PROJECT_NOT_FOUND", message: format!("Project not found: {id}") }
    }

    pub fn page_not_found(id: &str) -> Self {
        Self { code: "PAGE_NOT_FOUND", message: format!("Page not found: {id}") }
    }

    pub fn tag_not_found(id: &str) -> Self {
        Self { code: "TAG_NOT_FOUND", message: format!("Tag not found: {id}") }
    }

    // 鈹€鈹€ Configuration 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

    pub fn config_load(e: impl std::fmt::Display) -> Self {
        Self { code: "CONFIG_LOAD", message: format!("Configuration error: {e}") }
    }

    pub fn config_save(e: impl std::fmt::Display) -> Self {
        Self { code: "CONFIG_SAVE", message: format!("Failed to save configuration: {e}") }
    }

    pub fn config_update(e: impl std::fmt::Display) -> Self {
        Self { code: "CONFIG_UPDATE", message: format!("Failed to update configuration: {e}") }
    }

    // 鈹€鈹€ Internal 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

    pub fn internal(msg: impl Into<String>) -> Self {
        Self { code: "INTERNAL", message: msg.into() }
    }

    pub fn pool_init() -> Self {
        Self { code: "DB_INIT", message: "Database already initialized".into() }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}
