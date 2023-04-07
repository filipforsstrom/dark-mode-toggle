use std::process::Command;
use tauri_plugin_autostart::MacosLauncher;

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    // here `"quit".to_string()` defines the menu item id,
    // and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let auto_start_enabled =
        CustomMenuItem::new("auto_start_enabled".to_string(), "Auto Start Enabled");

    let tray_menu = SystemTrayMenu::new()
        // .add_item(auto_start_enabled)
        // .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new()
        .with_menu_on_left_click(false)
        .with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                toggle_dark_mode();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]), /* arbitrary number of args to pass to your app */
        ))
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn toggle_dark_mode() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode")
        .output()
        .expect("failed to execute process");
}
