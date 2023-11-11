use super::COUNTER;
use crate::app::Ctx;
use crate::png::draw_png;
use thruster::{middleware_fn, Context, MiddlewareNext, MiddlewareResult};

#[middleware_fn]
pub async fn ghstat(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let c: i32;

    {
        let mut counter = COUNTER.lock().await;
        counter.increment();
        c = counter.get_count();
    }

    context.set("Content-Type", "image/png");
    // return png image
    let buffer = draw_png(c).unwrap();
    context.set_body_bytes(buffer.try_into().unwrap());

    Ok(context)
}
