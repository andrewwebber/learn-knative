#[macro_use]
extern crate log;

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer};
use cloudevents::{EventBuilder, EventBuilderV10};
use cloudevents_sdk_actix_web::{HttpResponseBuilderExt, RequestExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    sequence: u8,
    message: String,
}

#[post("/")]
async fn reply_event(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let request_event = req.into_event(payload).await?;

    if let Ok(data) = request_event.try_get_data::<serde_json::Value>() {
        if let Some(data_inner) = data {
            if let Ok(ex) = serde_json::from_value::<Example>(data_inner) {
                info!("Event Data {:?}", ex);
            }
        }
    }

    // Build response event cloning the original event and setting the new type and source
    let response_event = EventBuilderV10::from(request_event)
        .source("https://rust-service")
        .ty("reply.rust")
        .build()
        // If i can't build the event, fail with internal server error
        .map_err(actix_web::error::ErrorInternalServerError)?;

    HttpResponseBuilder::new(StatusCode::OK)
        .event(response_event)
        .await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting rust-service");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::new().finish())
            .service(reply_event)
    })
    .bind("127.0.0.1:9000")?
    .workers(1)
    .run()
    .await
}
