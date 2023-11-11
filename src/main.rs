use log::info;
use std::time::Duration;
use thruster::{hyper_server::HyperServer, ThrusterServer};

mod app;
mod counter;
mod fileutil;
mod png;
mod rest;

#[macro_use]
pub extern crate lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref COUNTER: Mutex<counter::Counter> =
        Mutex::new(counter::Counter::new("counter.txt"));
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server...");

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

    fileutil::create_file("counter.txt", "0");
    fileutil::create_file("log.txt", "");

    let app = app::app().await.expect("Could not create app");

    let server = HyperServer::new(app);
    println!("Listening on http://0.0.0.0:3000");
    server.build("0.0.0.0", 3000).await;
}
