pub mod galaxy;
pub mod planet;
pub mod star;

#[macro_export]
macro_rules! impl_json_responder {
  ($response:ident, $status_code:expr) => {
    impl actix_web::Responder for $response {
      type Body = actix_web::body::BoxBody;

      fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::build($status_code).json(self)
      }
    }
  };
}

pub trait FromPath: Sized {
  fn from_path(path: actix_web::web::Path<Self>) -> Self {
    path.into_inner()
  }
}
