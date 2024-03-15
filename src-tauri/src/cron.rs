use std::sync::Arc;
use tokio::time::Duration;
use tokio::{self, time};
use tokio::sync::{mpsc, Mutex};
use crate::worker;

pub struct Cron {
  interval: Arc<Mutex<Duration>>,
  worker: Arc<Mutex<worker::Worker>>,
}

impl Cron {
    pub fn new(initial_interval: Duration) -> Arc<Self> {
      Arc::new(Self {
        interval: Arc::new(Mutex::new(initial_interval)),
        worker: Arc::new(Mutex::new(worker::Worker::new())),
      })
    }

    pub async fn start(self: Arc<Self>, mut rx: mpsc::Receiver<u64>) {
      tokio::spawn(async move {
        let mut current_interval = 1800; // 当前间隔时间（秒）

        loop {
          let interval = {
            let interval = self.interval.lock().await;
            *interval
          };

          let mut ticker = time::interval(interval);
          ticker.tick().await;

          tokio::select! {
            _ = ticker.tick(), if current_interval > 0 => {
              self.on_tick().await;
            },
            new_interval = rx.recv() => {
              match new_interval {
                Some(0) => {
                  current_interval = 0;
                },
                Some(sec) => {
                  current_interval = sec;
                  let mut interval = self.interval.lock().await;
                  *interval = Duration::from_secs(current_interval);
                },
                None => {
                  break;
                }
              }
          },
          }
        }
      });
    }

    async fn on_tick(&self) {
      self.worker.lock().await.next_image().await.unwrap();
    }
}
