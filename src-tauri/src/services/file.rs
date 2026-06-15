use crate::utils::error::AppError;
use crate::utils::config::Config;
use tauri::Manager;

/// File service 鈥?encapsulates business logic for file I/O operations.
pub struct FileService;

impl FileService {
    /// Export content as a Markdown file inside the workspace.
    pub async fn export_markdown(app: &tauri::AppHandle, content: String, filename: String) -> Result<String, AppError> {
        let config = app.state::<Config>();
        let export_dir = config.workspace_path().join("exports");
        std::fs::create_dir_all(&export_dir)
            .map_err(|e| AppError::internal(format!("Failed to create export dir: {e}")))?;

        let safe_name = sanitize_filename(&filename);
        let path = export_dir.join(format!("{safe_name}.md"));
        std::fs::write(&path, &content)
            .map_err(|e| AppError::internal(format!("Failed to write file: {e}")))?;

        log::info!("Exported markdown: {}", path.display());
        Ok(path.to_string_lossy().to_string())
    }
}

fn sanitize_filename(name: &str) -> String {
    let invalid = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    name.chars()
        .map(|c| if invalid.contains(&c) { '_' } else { c })
        .collect::<String>()
        .trim()
        .to_string()
}
