use std::time::Duration;
use super::COUNTER;
use crate::fileutil;

pub fn save_count() {
    tokio::spawn(async move {
        loop {
            {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let counter = COUNTER.lock().await;
                fileutil::write_count("counter.txt", counter.get_count());
                println!("Saved count: {}", counter.get_count());
            }
        }
    });
}
