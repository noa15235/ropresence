use crate::config::store;
use crate::state::AppState;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Ouvrir RoPresence", true, None::<&str>)?;
    let toggle = MenuItem::with_id(
        app,
        "toggle",
        "Activer / Désactiver la présence",
        true,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &toggle, &separator, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip("RoPresence")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_window(app),
            "toggle" => toggle_master(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_master(app: &AppHandle) {
    let state = app.state::<Arc<AppState>>();
    {
        let mut cfg = state.config.lock().unwrap();
        cfg.master_enabled = !cfg.master_enabled;
    }
    let snapshot = state.config.lock().unwrap().clone();
    let _ = store::save_config(app, &snapshot);
    state.notify_worker();
}
