use std::path::Path;
use std::fs;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use tauri;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
  pub key: String,
  pub interval: u64,
}

impl AppConfig {
  pub fn new() -> Self {
    let config = Self::get_initial_config();
    Self {
      key: config.key,
      interval: config.interval,
    }
  }

  pub fn get_initial_config() -> AppConfig {
    AppConfig {
      key: "bGqOr9yb7NN6t9ZN3pugAYyhhYL85wxM3yZFdM91hTM".to_owned(),
      interval: 0,
    }
  }

  pub fn create_app_folder () -> Result<String, Error> {
    let home_dir = tauri::api::path::home_dir();
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("tauri_scapes.toml");

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".tauri_scapes");

        match fs::create_dir_all(app_config_dir.clone()) {
          Ok(_) => {
            let content = toml::to_string(&Self::get_initial_config()).unwrap();
            fs::write(file_path, content).expect("write file error");
            Ok(app_config_dir.clone().to_str().unwrap().to_string())
          },
          Err(e) => Err(e)
        }
      }
      None => {
       Err(Error::new(ErrorKind::NotFound, "home dir is not fount"))
      }
    }
  }

  pub fn get_app_folder() -> Result<String, (usize, String)> {
    let home_dir = tauri::api::path::home_dir();

    match home_dir {
      Some(home_dir) => {
        let app_config_dir = Path::new(&home_dir).join(".tauri_scapes");

        if app_config_dir.exists() {
          Ok(app_config_dir.clone().to_str().unwrap().to_string())
        } else {
          Ok(Self::create_app_folder().unwrap())
        }
      }
      None => {
        Err((2, "no home dir".to_string()))
      }
    }
  }

  pub fn write_config(data: AppConfig) {
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("tauri_scapes.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).expect("create config failed");
    }

    let content = toml::to_string(&data).unwrap();

    fs::write(file_path, content).expect("write file error");
  }

  pub fn get_config() -> Self {
    let folder_dir = Self::get_app_folder().unwrap();
    let file_path = Path::new(&folder_dir).join("tauri_scapes.toml");

    if !file_path.exists() {
      fs::File::create(&file_path).expect("create config failed");
    }

    let content = match fs::read_to_string(&file_path) {
      Ok(content) => content,
      Err(_) => "".to_string(),
    };

    let data: AppConfig = match toml::from_str(&content) {
      Ok(data) => AppConfig { ..data },
      Err(_) => AppConfig::new()
    };

    data
  }
}
