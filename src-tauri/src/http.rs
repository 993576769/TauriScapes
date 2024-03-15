use reqwest::Client;
use crate::config;

#[allow(unused)]
pub async fn post(url: String, body: String) -> Result<reqwest::Response, reqwest::Error> {
  let mut req = Client::new().post(url);
  req = req.headers(build_header());
  let to = std::time::Duration::from_secs(30);
  req.body(body).timeout(to).send().await
}

pub async fn get(url: String) -> Result<reqwest::Response, reqwest::Error> {
  let mut req = Client::new().get(url);
  req = req.headers(build_header());
  let to = std::time::Duration::from_secs(30);
  req.timeout(to).send().await
}

fn build_header() -> reqwest::header::HeaderMap {
  let cfg: config::AppConfig = config::AppConfig::get_config();

  let mut headers = reqwest::header::HeaderMap::new();
  // match data {
  //   Some(data) => {
  //     let tmp: Vec<&str> = data.split(": ").collect();
  //     if tmp.len() == 2 {
  //       headers.insert(reqwest::header::HeaderName::from_bytes(tmp[0].as_bytes()).unwrap(), reqwest::header::HeaderValue::from_bytes(tmp[1].as_bytes()).unwrap());
  //     }
  //   },
  //   None => {}
  // }
  // if !data.is_empty() {
  //     let tmp: Vec<&str> = data.split(": ").collect();
  //     if tmp.len() == 2 {
  //       headers.insert(reqwest::header::HeaderName::from_bytes(tmp[0].as_bytes()).unwrap(), reqwest::header::HeaderValue::from_bytes(tmp[1].as_bytes()).unwrap());
  //     }
  //   }
  headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
  let auth_value = format!("Client-ID {}", cfg.key);
  headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&auth_value).unwrap());
  headers
}
