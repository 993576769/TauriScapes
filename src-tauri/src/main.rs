#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod command;
mod config;
mod scheduler;
mod http;
mod tray;
mod models;

use tauri::Manager;
use tokio::sync::{mpsc, Mutex};
use dotenv::dotenv;
use crate::scheduler::AsyncProcessMessage;
use crate::command::AsyncProcInputTx;

#[tokio::main]
async fn main() {
  dotenv().ok();
  config::AppConfig::create_app_folder().expect("create app folder failed!");

  let (async_process_input_tx, mut async_process_input_rx) = mpsc::channel::<AsyncProcessMessage>(32);
  let tx = async_process_input_tx.clone();

  let mut scheduler: scheduler::Scheduler = scheduler::Scheduler::new();

  tauri::async_runtime::spawn(async move {
    loop {
      if let Some(message) = async_process_input_rx.recv().await {
        match message {
          AsyncProcessMessage::NextImage => {
            scheduler.next_image().await.unwrap();
          }
        }
      }
    }
  });

  tauri::Builder::default()
    .manage(AsyncProcInputTx {
      sender: Mutex::new(async_process_input_tx),
    })
    .plugin(tauri_plugin_positioner::init())
    .system_tray(tray::create_tray())
    .on_system_tray_event(move |app, event| {
      tray::handle_tray_event(app, event, tx.clone())
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
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

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
      command::get_config,
      command::write_config,
      command::set_interval,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
