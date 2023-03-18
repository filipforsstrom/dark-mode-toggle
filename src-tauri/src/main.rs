use std::process::Command;

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
                toggle_dark_mode();
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_dark_mode() {
    Command::new("sh")
        .arg("-c")
        .arg("/Users/panopticon/dev/rust/dark-mode-toggle/src-tauri/scripts/osascript.sh")
        .output()
        .expect("failed to execute osascript");
}
