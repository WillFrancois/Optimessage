use actix_web::{
    App, HttpResponseBuilder, HttpServer, Responder, http::StatusCode, post, web::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct MessageVerification {
    message: String,
}

#[post("/verify-message")]
async fn greet(info: Json<MessageVerification>) -> impl Responder {
    println!("{}", info.message);
    HttpResponseBuilder::new(StatusCode::from_u16(400).unwrap())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
