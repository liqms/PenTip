use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
#[allow(unused_variables)]
pub async fn create_note(_app: tauri::AppHandle, _content: String, _note_type: String) -> Result<String, String> {
    let id = format!("note_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
    // TODO: 写入 SQLite
    log::info!("Created note: {}", id);
    Ok(id)
}
