use argon2::{
  password_hash::{
    rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
  },
  Argon2,
};
use serde::Deserialize;
use validator::Validate;

use super::{AuthError, AuthResult, AuthSecurity};

#[derive(Debug, Deserialize, Validate)]
pub struct Password {
  #[serde(rename = "password")]
  #[validate(length(min = 1, message = "cannot be empty"))]
  value: String,
}

impl From<Error> for AuthError {
  fn from(value: Error) -> Self {
    match value {
      Error::Password => AuthError::Invalid,
      err => AuthError::Other(err.to_string()),
    }
  }
}

impl AuthSecurity for Password {
  fn hash(&self) -> AuthResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
      .hash_password(self.value.as_bytes(), &salt)?
      .to_string();

    Ok(password_hash)
  }

  fn verify(&self, hash: &str) -> AuthResult<()> {
    let parsed_hash = PasswordHash::new(&hash)?;

    let argon2 = Argon2::default();

    argon2.verify_password(self.value.as_bytes(), &parsed_hash)?;

    Ok(())
  }
}
