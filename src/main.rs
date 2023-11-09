use log::info;
use thruster::{hyper_server::HyperServer, ThrusterServer};

mod app;
mod counter;
mod rest;

#[macro_use]
pub extern crate lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref COUNTER: Mutex<counter::Counter> = Mutex::new(counter::Counter::new());
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server...");

    let app = app::app().await.expect("Could not create app");

    let server = HyperServer::new(app);
    server.build("0.0.0.0", 3000).await;
}
