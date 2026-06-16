#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod discord;
mod presence;
mod roblox;
mod state;
mod tray;

use crate::state::AppState;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::set_config,
            commands::get_runtime,
            commands::get_variables,
            commands::toggle_master,
            commands::reconnect_discord,
            commands::connect_roblox,
            commands::validate_client_id,
            commands::open_url,
            commands::get_logs,
            commands::clear_logs,
            commands::export_config,
            commands::import_config,
            commands::show_main_window,
            commands::quit_app,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            let cfg = config::store::load_config(&handle);
            let start_minimized = cfg.system.start_minimized;
            let state = Arc::new(AppState::new(cfg));
            app.manage(state.clone());

            tray::create_tray(&handle)?;

            if start_minimized {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            let worker_handle = handle.clone();
            let worker_state = state.clone();
            std::thread::spawn(move || presence::run_worker(worker_handle, worker_state));

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                let close_to_tray = app
                    .state::<Arc<AppState>>()
                    .config
                    .lock()
                    .map(|c| c.system.close_to_tray)
                    .unwrap_or(true);
                if close_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
