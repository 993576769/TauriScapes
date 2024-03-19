#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod command;
mod config;
mod worker;
mod http;
mod tray;
mod models;
mod cron;

use tauri::Manager;
use tokio::sync::{mpsc, Mutex};
use crate::worker::WorkerMessage;
use tokio::time::Duration;
use tauri_plugin_autostart::MacosLauncher;

pub struct AsyncProcInputTx {
  pub worker_sender: Mutex<mpsc::Sender<WorkerMessage>>,
  pub cron_sender: Mutex<mpsc::Sender<u64>>,
}

struct Senders {
  pub worker_sender: mpsc::Sender<WorkerMessage>,
  pub cron_sender: mpsc::Sender<u64>,
}

fn init_tauri() {
  let (async_process_input_tx, mut async_process_input_rx) = mpsc::channel::<WorkerMessage>(32);
  let async_tx = async_process_input_tx.clone();

  tokio::spawn(async move {
    let mut worker: worker::Worker = worker::Worker::new();
    loop {
      if let Some(message) = async_process_input_rx.recv().await {
        match message {
          WorkerMessage::NextImage => {
            worker.next_image().await.unwrap();
          }
        }
      }
    }
  });

  let (cron_input_tx, cron_input_rx) = mpsc::channel::<u64>(32);
  let cron_tx = cron_input_tx.clone();

  let interval = if config::AppConfig::get_config().interval > 0 {
    Duration::from_secs(config::AppConfig::get_config().interval)
  } else {
    Duration::from_secs(1800)
  };
  let cron = cron::Cron::new(interval);
  tokio::spawn(async move {
    cron.clone().start(cron_input_rx).await;
  });

  tauri::Builder::default()
    .manage(AsyncProcInputTx {
      worker_sender: Mutex::new(async_process_input_tx),
      cron_sender: Mutex::new(cron_input_tx),
    })
    .plugin(tauri_plugin_positioner::init())
    .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
    .system_tray(tray::create_tray())
    .on_system_tray_event(move |app, event| {
      tray::handle_tray_event(app, event, {
        Senders {
          worker_sender: async_tx.clone(),
          cron_sender: cron_tx.clone(),
        }
      })
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
      window_vibrancy::apply_vibrancy(&window, window_vibrancy::NSVisualEffectMaterial::HudWindow, None, Some(8.0))
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
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tokio::main]
async fn main() {
  config::AppConfig::get_app_folder().expect("create app folder failed!");
  init_tauri();
}
