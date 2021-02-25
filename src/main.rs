use actix_web::{post, web, App, HttpServer, Responder};
use dotenv;

mod http_client;
mod id_to_member;
mod types;

use http_client::post_face;
use types::*;

#[post("/")]
async fn post(request_body: web::Json<DetectRequest>) -> impl Responder {
    let similar_member_list = post_face(&request_body.url).await;
    web::Json(similar_member_list)
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv::dotenv().ok();
    HttpServer::new(move || App::new().service(post))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}