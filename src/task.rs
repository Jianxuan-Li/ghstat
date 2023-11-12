use super::COUNTER;
use super::LOG_QUEUE;
use crate::fileutil;
use std::time::Duration;

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

pub fn save_log() {
    tokio::spawn(async move {
        loop {
            {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut log_queue = LOG_QUEUE.lock().await;
                let mut logs = Vec::new();
                while let Some(log) = log_queue.pop() {
                    logs.push(log);
                }
                println!("Saved logs: {} lines", logs.len());
                fileutil::append_logs("log.txt", logs);
            }
        }
    });
}
