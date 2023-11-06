use log::info;
use thruster::{
    context::typed_hyper_context::TypedHyperContext, hyper_server::HyperServer, m, App,
    HyperRequest, ThrusterServer,
};

mod counter;
mod rest;

#[macro_use]
pub extern crate lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref COUNTER: Mutex<counter::Counter> = Mutex::new(counter::Counter::new());
}

type Ctx = TypedHyperContext<RequestConfig>;

#[derive(Default)]
struct ServerConfig {}

#[derive(Default)]
pub struct RequestConfig {}

fn generate_context(request: HyperRequest, _state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(request, RequestConfig {})
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server...");

    use rest::ghstat;
    let mw = m![ghstat];

    let app = App::<HyperRequest, Ctx, ServerConfig>::create(generate_context, ServerConfig {})
        .get("/stat", mw);

    let server = HyperServer::new(app);
    server.build("0.0.0.0", 4321).await;
}
