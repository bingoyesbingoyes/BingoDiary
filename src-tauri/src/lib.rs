mod commands;
#[cfg(desktop)]
mod menu;
mod diary;
mod config;
mod pdf;
mod tags;
mod sync;

#[cfg(desktop)]
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_notification::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Build application menu (desktop only)
      #[cfg(desktop)]
      {
        let app_handle = app.handle();
        let menu = menu::build_menu(&app_handle)?;
        app.set_menu(menu)?;

        // Handle menu events
        app.on_menu_event(move |app, event| {
          match event.id().as_ref() {
            "export-json" => { let _ = app.emit("menu-export-json", ()); },
            "export-pdf" => { let _ = app.emit("menu-export-pdf", ()); },
            "export-markdown" => { let _ = app.emit("menu-export-markdown", ()); },
            "load-json" => { let _ = app.emit("menu-load-json", ()); },
            "open-settings" => { let _ = app.emit("menu-open-settings", ()); },
            _ => {}
          }
        });
      }

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::save_diary,
      commands::load_diary,
      commands::get_all_diaries,
      commands::save_image,
      commands::export_json,
      commands::import_json,
      commands::export_images,
      commands::import_images,
      commands::get_storage_path,
      commands::change_storage_path,
      commands::get_app_data_dir,
      commands::export_pdf,
      // Settings commands
      commands::get_config,
      commands::save_config,
      commands::set_password,
      commands::verify_password,
      commands::clear_password,
      commands::has_password,
      commands::save_background_image,
      // Tag commands
      commands::create_tag,
      commands::update_tag,
      commands::delete_tag,
      commands::get_all_tags,
      commands::set_entry_tags,
      commands::get_entry_tags,
      commands::get_entries_by_tag,
      commands::get_tag_stats,
      // Schedule event commands
      commands::get_events_for_date,
      commands::save_event,
      commands::delete_event,
      // Sync commands
      commands::get_sync_status,
      commands::start_sync,
      commands::force_upload_sync,
      commands::get_google_auth_url,
      commands::handle_oauth_callback,
      commands::disconnect_google,
      commands::get_sync_settings,
      commands::save_sync_settings,
      commands::save_google_client_id,
      commands::save_google_credentials,
      commands::get_google_client_id,
      commands::clear_sync_credentials,
      #[cfg(desktop)]
      commands::start_oauth_callback_server,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
