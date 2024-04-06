use actix_web::{App, HttpServer};
use api::endpoints::{galaxy, planet, star};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(galaxy::list_all_galaxies)
      .service(galaxy::create_galaxy)
      .service(galaxy::update_galaxy)
      .service(galaxy::delete_galaxy)
      .service(star::list_all_stars)
      .service(star::create_star)
      .service(star::update_star)
      .service(star::delete_star)
      .service(planet::list_all_planets)
      .service(planet::create_planet)
      .service(planet::update_planet)
      .service(planet::delete_planet)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
