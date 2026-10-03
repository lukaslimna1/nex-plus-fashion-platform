#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

pub fn run() {
    let state = core::state::CoreState::new().expect("initialize local core database");

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            core::commands::core_health,
            core::commands::core_migrations
        ])
        .run(tauri::generate_context!())
        .expect("error while running NEX+ Fashion");
}
