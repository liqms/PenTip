use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind, RotationStrategy, TimezoneStrategy};

pub mod commands;
pub mod db;
pub mod services;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .level_for("sqlx", log::LevelFilter::Warn)
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::LogDir { file_name: None }))
                .rotation_strategy(RotationStrategy::KeepAll)
                .max_file_size(500_000)
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .build(),
        )
        .setup(|app| {
            log::info!("PenTip starting up");

            // Load all config files (app.json, shortcuts.json, …)
            // Creates defaults if files don't exist.
            let config = utils::config::Config::load(app)
                .unwrap_or_else(|e| {
                    log::error!("Failed to load config: {e}");
                    panic!("{e}")
                });
            log::info!("Workspace: {}", config.workspace);

            // Initialize database pool inside the workspace
            let db_url = config.db_url();
            tauri::async_runtime::block_on(async {
                db::Database::init(&db_url)
                    .await
                    .unwrap_or_else(|e| {
                        log::error!("Failed to initialize database: {e}");
                        panic!("{e}")
                    });
            });
            log::info!("Database initialized");

            // Make config available to commands via Tauri state
            app.manage(config);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fragment::create_fragment,
            commands::config::get_config,
            commands::config::update_app_config,
            commands::config::update_shortcuts_config,
            commands::config::set_workspace,
            commands::config::reset_config,
            commands::fragment::list_fragments,
            commands::fragment::get_fragment,
            commands::fragment::update_fragment,
            commands::fragment::delete_fragment,
            commands::fragment::search_fragments,
            commands::tag::create_tag,
            commands::tag::list_tags,
            commands::tag::delete_tag,
            commands::tag::attach_tag,
            commands::tag::detach_tag,
            commands::tag::list_tags_for_target,
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::update_project,
            commands::project::delete_project,
            commands::page::create_page,
            commands::page::list_pages,
            commands::page::update_page,
            commands::page::delete_page,
            commands::file::export_markdown,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PenTip");
}

