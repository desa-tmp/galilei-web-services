mod middleware;
mod password;
mod token;

pub use middleware::AuthService;
pub use password::Password;
pub use token::Token;

pub type AuthResult<T> = std::result::Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
  Invalid,
  Other(String),
}

pub trait AuthSecurity {
  fn hash(&self) -> AuthResult<String>;

  fn verify(&self, hash: &str) -> AuthResult<()>;
}
