
use tauri;
use std::{fs::File, io::copy, path::PathBuf};
use anyhow::Result;
use wallpaper;
use dirs_next;

const CACHE_DIR: &str = "TauriScapes/cache";

#[tauri::command]
pub async fn set_wallpaper(url: &str, file_name: &str) -> Result<(), String> {
  let cache_dir = match dirs_next::cache_dir() {
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
  let download_dir = match dirs_next::download_dir() {
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
