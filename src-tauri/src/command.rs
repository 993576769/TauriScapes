
use tauri;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::{fs::File, io::copy, env};
use anyhow::Result;
use wallpaper;

#[tauri::command]
pub async fn set_wallpaper(url: &str) -> Result<(), String> {
  let response = reqwest::get(url).await;
  let data = match response {
    Ok(data) => data,
    Err(_) => return Err("Could not fetch image".to_string())
  };

  let path: &str = &format!("{}/unsplash_{}.jpg", env::temp_dir().display(), random_string(16));
  let mut file = File::create(path).unwrap();
  let content = data.bytes().await.unwrap();
  copy(&mut content.as_ref(), &mut file).unwrap();
  let _ = wallpaper::set_from_path(path);
  Ok(())
}

fn random_string(n: usize) -> String {
  thread_rng().sample_iter(&Alphanumeric)
    .take(n)
    .map(char::from)
    .collect()
}
