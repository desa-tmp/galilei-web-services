use api::openapi::ApiSpec;
use std::fs;
use utoipa::OpenApi;

fn main() {
  fs::write(
    "./api/openapi.json",
    ApiSpec::openapi().to_pretty_json().unwrap(),
  )
  .unwrap();
}
