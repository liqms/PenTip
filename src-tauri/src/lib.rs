mod commands;
mod db;
mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好，{}！欢迎使用 PenTip", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet, commands::notes::create_note, commands::file::export_markdown])
        .run(tauri::generate_context!())
        .expect("error while running PenTip");
}
