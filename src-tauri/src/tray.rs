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
) -> tauri::Result<()> {
  let next_photo_i = MenuItem::with_id(app, "switch-wallpaper", "Switch wallpaper", true, None::<&str>)?;
  let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
  let menu1 = Menu::with_items(
    app,
    &[
      &next_photo_i,
      &PredefinedMenuItem::separator(app)?,
      &quit_i,
    ],
  )?;

  let _ = TrayIconBuilder::with_id("tray")
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu1)
    .menu_on_left_click(false)
    .on_menu_event(move |app, event| match event.id.as_ref() {
      "switch-wallpaper" => {
        let tx = senders.worker_sender.clone();
        tokio::spawn(async move {
          tx.send(WorkerMessage::NextImage).await.unwrap();
        });
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
