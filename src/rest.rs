use super::COUNTER;
use crate::app::Ctx;
use thruster::{middleware_fn, MiddlewareNext, MiddlewareResult};

// 5
#[middleware_fn]
pub async fn ghstat(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let c: i32;

    {
        let mut counter = COUNTER.lock().await;
        counter.increment();
        c = counter.get_count();
    }

    println!("Count: {}", c);

    context.body("Hello, world!");

    Ok(context)
}
