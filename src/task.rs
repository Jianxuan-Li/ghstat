use super::COUNTER;
use super::LOG_QUEUE;
use crate::fileutil;
use std::time::Duration;

pub fn save_count() {
    tokio::spawn(async move {
        loop {
            {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut counter = COUNTER.lock().await;
                let curr_count = counter.get_count();

                if curr_count == counter.get_prev_count() {
                    // prevent IO if count is not changed
                    continue;
                }

                counter.set_prev_count(curr_count);
                fileutil::write_count("counter.txt", curr_count);
                println!("Saved count: {}", curr_count);
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

                if log_queue.size() == 0 {
                    // prevent IO if log queue is empty
                    continue;
                }

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
