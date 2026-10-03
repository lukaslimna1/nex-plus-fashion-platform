#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let database_path = app
                .path()
                .app_local_data_dir()
                .expect("resolve local app data directory")
                .join("catalog.sqlite");
            let state = core::state::CoreState::open(database_path)
                .expect("initialize local core database");
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::commands::core_health,
            core::commands::core_migrations,
            core::commands::catalog_milano_ss27,
            core::commands::catalog_search,
            core::commands::personal_note_upsert
        ])
        .run(tauri::generate_context!())
        .expect("error while running NEX+ Fashion");
}
