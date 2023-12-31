use log::info;
use thruster::{hyper_server::HyperServer, ThrusterServer};

mod app;
mod counter;
mod fileutil;
mod png;
mod queue;
mod rest;
mod task;

#[macro_use]
pub extern crate lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref COUNTER: Mutex<counter::Counter> =
        Mutex::new(counter::Counter::new("counter.txt"));
    pub static ref LOG_QUEUE: Mutex<queue::LogQueue> = Mutex::new(queue::LogQueue::new());
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server...");

    // create files if not exist
    fileutil::create_file("counter.txt", "0");
    fileutil::create_file("log.txt", "");

    // active tasks in other threads
    task::save_count();
    task::save_log();

    // create app
    let app = app::app().await.expect("Could not create app");
    let server = HyperServer::new(app);
    println!("Listening on http://0.0.0.0:3000");
    server.build("0.0.0.0", 3000).await;
}
