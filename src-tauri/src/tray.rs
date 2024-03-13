use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, AppHandle};
use tokio::sync::mpsc;
use tauri_plugin_positioner::{Position, WindowExt};
use crate::scheduler::AsyncProcessMessage;
use tauri::Manager;

pub fn create_tray() -> SystemTray {
  let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
  let next_photo: CustomMenuItem = CustomMenuItem::new("next_photo".to_string(), "Next Photo");

  let tray_menu = SystemTrayMenu::new()
    .add_item(next_photo)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(
  app: &AppHandle,
  event: SystemTrayEvent,
  sender: mpsc::Sender<AsyncProcessMessage>,
) {
  match event {
    SystemTrayEvent::LeftClick {
      position: _,
      size: _,
      ..
    } => {
      tauri_plugin_positioner::on_tray_event(app, &event);

      let window = app.get_window("main").unwrap();

      #[cfg(target_os = "windows")]
      let _ = window.move_window(Position::Center);

      #[cfg(not(target_os = "windows"))]
      let _ = window.move_window(Position::TrayBottomCenter);

      if window.is_visible().unwrap() {
        window.hide().unwrap();
      } else {
        window.show().unwrap();
        window.set_focus().unwrap();
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
      match id.as_str() {
        "next_photo" => {
          let tx = sender.clone();
          tokio::spawn(async move {
            tx.send(AsyncProcessMessage::NextImage).await.unwrap();
          });
        }
        "quit" => {
          std::process::exit(0);
        }
        _ => {}
      }
    }
    _ => {}
  };
}

