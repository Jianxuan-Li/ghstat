use crate::rest::ghstat;
use crate::rest::estat;
use thruster::context::typed_hyper_context::TypedHyperContext;
use thruster::{m, App, HyperRequest};

#[derive(Default)]
pub struct ServerConfig {}

#[derive(Default)]
pub struct RequestConfig {}

pub type Ctx = TypedHyperContext<RequestConfig>;

fn generate_context(request: HyperRequest, _state: &ServerConfig, _path: &str) -> Ctx {
    Ctx::new(request, RequestConfig {})
}

pub async fn app() -> Result<App<HyperRequest, Ctx, ServerConfig>, Box<dyn std::error::Error>> {
    let app = App::<HyperRequest, Ctx, ServerConfig>::create(generate_context, ServerConfig {})
        .get("/v1/s", m![ghstat])
        .get("/v1/e", m![estat]);

    Ok(app)
}
