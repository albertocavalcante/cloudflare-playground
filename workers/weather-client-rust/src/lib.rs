use worker::*;
mod weather;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
  
    let router = Router::new();

    router
        .get_async("/weather/:city", |_, ctx| async move {
            if let Some(city) = ctx.param("city") {
                match weather::get_weather(city).await {
                    Ok(weather) => Ok(Response::from_json(&weather)?),
                    Err(e) => Ok(Response::error(e.to_string(), 500)?),
                }
            } else {
                Ok(Response::error("City parameter is required", 400)?)
            }
        })
        .run(req, env)
        .await
}
