use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

const MAX_CONNECTIONS: u32 = 10;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "debug");
  env::set_var("RUST_BACKTRACE", "1");

  dotenv().ok();

  env_logger::init();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let pool = api::database::create_pool(&database_url, MAX_CONNECTIONS)
    .await
    .expect("Successfully connect to database");

  HttpServer::new(move || {
    let logger = Logger::default();

    App::new()
      .app_data(web::Data::new(pool.clone()))
      .wrap(logger)
      .configure(api::routes::galaxy::config)
      .configure(api::routes::star::config)
      .configure(api::routes::planet::config)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
