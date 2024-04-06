use crate::endpoints::{galaxy, planet, star};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    galaxy::list_all_galaxies,
    galaxy::create_galaxy,
    galaxy::update_galaxy,
    galaxy::delete_galaxy,
    star::list_all_stars,
    star::create_star,
    star::update_star,
    star::delete_star,
    planet::list_all_planets,
    planet::create_planet,
    planet::update_planet,
    planet::delete_planet
  ),
  components(schemas(
    galaxy::Galaxy,
    galaxy::CreateGalaxyData,
    galaxy::UpdateGalaxyData,
    star::Star,
    star::CreateStarData,
    star::UpdateStarData,
    planet::Planet,
    planet::CreatePlanetData,
    planet::UpdatePlanetData,
    planet::ConnectPlanetToStar
  ))
)]
pub struct ApiSpec;
