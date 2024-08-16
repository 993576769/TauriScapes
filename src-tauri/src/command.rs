
use tauri;
use std::{fs::File, io::copy, path::PathBuf};
use anyhow::Result;
use wallpaper;
use crate::config;
use crate::AsyncProcInputTx;
use dirs;

const CACHE_DIR: &str = "tauri_scapes/cache";

#[tauri::command]
pub async fn set_wallpaper(url: &str, file_name: &str) -> Result<(), String> {
  let cache_dir = match dirs::cache_dir() {
    Some(path) => path,
    None => {
      return Err("Could not find download directory".to_string())
    }
  };

  // mkdir if not exists
  let path = cache_dir.join(CACHE_DIR);
  let _ = std::fs::create_dir_all(cache_dir.join(CACHE_DIR));

  let path = download(url, file_name, path.clone()).await.unwrap();
  let _ = wallpaper::set_from_path(&path);
  Ok(())
}

#[tauri::command]
pub async fn save_wallpaper(url: &str, file_name: &str) -> Result<String, String> {
  let download_dir = match dirs::download_dir() {
    Some(path) => path,
    None => {
      return Err("Could not find download directory".to_string())
    }
  };
  let path = download(url, file_name, download_dir).await.unwrap();
  let _ = wallpaper::set_from_path(&path);
  Ok(path)
}

async fn download(url: &str, file_name: &str, path: PathBuf) -> Result<String, String> {
  let response = reqwest::get(url).await;
  let data = match response {
    Ok(data) => data,
    Err(_) => return Err("Could not fetch image".to_string())
  };
  let path = path.join(format!("{}.jpg", file_name));
  let mut file = File::create(path.clone()).unwrap();
  let content = data.bytes().await.unwrap();
  copy(&mut content.as_ref(), &mut file).unwrap();
  Ok(path.display().to_string())
}

#[tauri::command]
pub async fn get_config() -> serde_json::Value {
  let app_config = config::AppConfig::get_config();

  serde_json::to_value(app_config).unwrap()
}

#[tauri::command]
pub async fn write_config(data: config::AppConfig, state: tauri::State<'_, AsyncProcInputTx>) -> Result<config::AppConfig, ()> {
  let config = config::AppConfig::get_config();
  config::AppConfig::write_config(data.clone());
  if data.interval != config.interval {
    let sender = state.cron_sender.lock().await;
    let _ = sender.send(data.interval).await;
  }
  Ok(data)
}
