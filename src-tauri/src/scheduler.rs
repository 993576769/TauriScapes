use serde::{Deserialize, Serialize};
use crate::{config, models::random::Random, http};
use std::{fs::File, io::copy, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub enum AsyncProcessMessage {
  NextImage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scheduler {
  pub interval: u64,
}

impl Scheduler {
  const CACHE_DIR: &'static str = "tauri_scapes/cache";

  pub fn new() -> Self {
    let cfg: config::AppConfig = config::AppConfig::get_config();

    Self {
      interval: cfg.interval,
    }
  }

  pub async fn download(&mut self, url: &str, file_name: &str, path: PathBuf) -> Result<String, String> {
    let response = http::get(url.to_string());
    let data = match response.await {
      Ok(data) => data,
      Err(_) => return Err("Could not fetch image".to_string())
    };
    let path = path.join(format!("{}.jpg", file_name));
    let mut file = File::create(path.clone()).unwrap();
    let content = data.bytes().await.unwrap();
    copy(&mut content.as_ref(), &mut file).unwrap();
    Ok(path.display().to_string())
  }

  pub async fn set_wallpaper(&mut self, url: &str, file_name: &str) -> Result<(), String> {
    let cache_dir = match tauri::api::path::cache_dir() {
      Some(path) => path,
      None => {
        return Err("Could not find download directory".to_string())
      }
    };

    // mkdir if not exists
    let path = cache_dir.join(Self::CACHE_DIR);
    let _ = std::fs::create_dir_all(cache_dir.join(Self::CACHE_DIR));

    let path = Self::download(self, url, file_name, path.clone()).await.unwrap();
    let _ = wallpaper::set_from_path(&path);
    Ok(())
  }

  pub async fn next_image(&mut self) -> Result<(), String> {
    let res = http::get("https://api.unsplash.com/photos/random".to_string()).await.unwrap();
    let data = match res.json::<Random>().await {
      Ok(data) => data,
      Err(_) => return Err("json parse fail".to_string())
    };
    Self::set_wallpaper(self, &data.urls.raw, &data.id).await.unwrap();
    Ok(())
  }
}
