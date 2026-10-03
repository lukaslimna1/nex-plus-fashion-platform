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
            core::commands::ai_health,
            core::commands::ai_executions,
            core::commands::curator_ai_run,
            core::commands::curator_proposals,
            core::commands::curator_proposal_decide,
            core::commands::catalog_milano_ss27,
            core::commands::catalog_search,
            core::commands::personal_note_upsert,
            core::commands::packs_list,
            core::commands::pack_install,
            core::commands::pack_remove,
            core::commands::pack_repair,
            core::commands::catalog_geography,
            core::commands::catalog_city_hubs,
            core::commands::catalog_city_hub,
            core::commands::catalog_events,
            core::commands::catalog_event,
            core::commands::catalog_editions,
            core::commands::catalog_edition,
            core::commands::catalog_schedule,
            core::commands::catalog_venues,
            core::commands::catalog_venue,
            core::commands::catalog_maisons,
            core::commands::catalog_maison,
            core::commands::catalog_persons,
            core::commands::catalog_person,
            core::commands::catalog_roles,
            core::commands::catalog_person_roles,
            core::commands::catalog_collections,
            core::commands::catalog_collection,
            core::commands::catalog_looks,
            core::commands::catalog_media,
            core::commands::catalog_media_availability,
            core::commands::catalog_reviews,
            core::commands::catalog_sources,
            core::commands::catalog_source,
            core::commands::catalog_provenance,
            core::commands::catalog_terms,
            core::commands::personal_related,
            core::commands::personal_favorite_set
        ])
        .run(tauri::generate_context!())
        .expect("error while running NEX+ Fashion");
}
