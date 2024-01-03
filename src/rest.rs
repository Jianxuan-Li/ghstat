use super::COUNTER;
use super::LOG_QUEUE;
use crate::app::Ctx;
use crate::png::draw_png;
use thruster::{
    middleware::cookies::HasCookies, middleware_fn, Context, MiddlewareNext, MiddlewareResult,
};

#[middleware_fn]
pub async fn ghstat(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let c: i32;

    {
        let mut counter = COUNTER.lock().await;
        counter.increment();
        c = counter.get_count();

        // get ip address, user agent, and referer from request header
        let user_agent = context
            .get_header("user-agent")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let referer = context
            .get_header("referer")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let ip = context
            .get_header("x-forwarded-for")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let current_time_str = chrono::Local::now().to_string();
        let log_str = format!(
            "{}\t{}\t{}\t{}\t{}",
            current_time_str, ip, user_agent, referer, c
        );
        println!("{}", log_str);

        LOG_QUEUE.lock().await.push(log_str);
    }

    // set response header
    context.remove("server");
    context.set("Content-Type", "image/png");
    // disable cache
    context.set("Cache-Control", "no-cache, no-store, must-revalidate");

    // return png image
    let buffer = draw_png(c).unwrap();
    context.set_body_bytes(buffer.try_into().unwrap());

    Ok(context)
}

#[middleware_fn]
pub async fn estat(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let c: i32;

    {
        let mut counter = COUNTER.lock().await;
        counter.increment();
        c = counter.get_count();

        // get ip address, user agent, and referer from request header
        let user_agent = context
            .get_header("user-agent")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let referer = context
            .get_header("referer")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let ip = context
            .get_header("x-forwarded-for")
            .pop()
            .unwrap_or_else(|| "".to_string());
        let current_time_str = chrono::Local::now().to_string();
        let host = context
            .get_header("host")
            .pop()
            .unwrap_or_else(|| "".to_string());

        let log_str = format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            host, current_time_str, ip, user_agent, referer, c
        );
        println!("{}", log_str);

        LOG_QUEUE.lock().await.push(log_str);
    }

    context.remove("server");
    context.set("Cache-Control", "no-cache, no-store, must-revalidate");
    
    // return json with counter
    let content = format!("{{\"count\":{}}}", &c);
    context.body(&content);

    Ok(context)
}