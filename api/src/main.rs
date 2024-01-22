use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use api;

const fn default_i32<const V: i32>() -> i32 {
  V
}

#[derive(Debug, Deserialize)]
struct Addition {
  #[serde(default = "default_i32::<2>")]
  left: i32,
  #[serde(default = "default_i32::<2>")]
  right: i32,
}

#[get("/")]
async fn hello_world() -> impl Responder {
  HttpResponse::Ok().body("Hello, World!")
}

#[get("/add")]
async fn add(web::Query(Addition { left, right }): web::Query<Addition>) -> impl Responder {
  HttpResponse::Ok().body(format!("{left} + {right} = {}", api::add(left, right)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(hello_world).service(add))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
