use tauri::{
  menu::{Menu, MenuItem, PredefinedMenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  Manager, Runtime,
};
use tauri_plugin_positioner::{Position, WindowExt};
use crate::Senders;
use crate::worker::WorkerMessage;

pub fn create_tray<R: Runtime>(
  app: &tauri::AppHandle<R>,
  senders: Senders,
) {
  tauri_plugin_positioner::on_tray_event(app, &event);
  match event {
    SystemTrayEvent::LeftClick {
      position: _,
      size: _,
      ..
    } => {
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
      "quit" => {
        app.exit(0);
      }
      _ => {}
    })
    .on_tray_icon_event(|app, event| {
      tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } = event
      {
        let app = app.app_handle();
        if let Some(window) = app.get_webview_window("main") {
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
      }
    })
    .build(app);

  Ok(())
}
