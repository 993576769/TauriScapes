#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod command;

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, ActivationPolicy};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
  let system_tray_menu = SystemTrayMenu::new().add_item(quit);
  tauri::Builder::default()
      .plugin(tauri_plugin_positioner::init())
      .system_tray(SystemTray::new().with_menu(system_tray_menu))
      .on_system_tray_event(|app, event| {
          tauri_plugin_positioner::on_tray_event(app, &event);
          match event {
              SystemTrayEvent::LeftClick {
                  position: _,
                  size: _,
                  ..
              } => {
                  let window = app.get_window("main").unwrap();
                  let _ = window.move_window(Position::TrayBottomCenter);

                  if window.is_visible().unwrap() {
                      window.hide().unwrap();
                  } else {
                      window.show().unwrap();
                      window.set_focus().unwrap();
                  }
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
          }
      })
      .on_window_event(|event| match event.event() {
          tauri::WindowEvent::Focused(is_focused) => {
              // detect click outside of the focused window and hide the app
              if !is_focused {
                  event.window().hide().unwrap();
              }
          }
          _ => {}
      })
      .setup(|app| {
        let window = app.get_window("main").unwrap();
        // hide dock icon on macos
        app.set_activation_policy(ActivationPolicy::Accessory);

        #[cfg(target_os = "macos")]
        window_vibrancy::apply_vibrancy(&window, window_vibrancy::NSVisualEffectMaterial::HudWindow, None, None)
          .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        #[cfg(target_os = "windows")]
        window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
          .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        command::set_wallpaper,
        command::save_wallpaper,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
