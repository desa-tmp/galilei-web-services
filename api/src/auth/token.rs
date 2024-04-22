use derive_more::From;
use sha2::{Digest, Sha256};

use super::{AuthError, AuthResult, AuthSecurity};

#[derive(Debug, From)]
pub struct Token(String);

impl Token {
  const LENGTH: usize = 32;

  pub fn new(value: String) -> Self {
    Self(value)
  }

  pub fn generate() -> AuthResult<Self> {
    let mut token_buf = [0u8; Token::LENGTH];
    getrandom::getrandom(&mut token_buf).map_err(|err| AuthError::Other(err.to_string()))?;

    let token = hex::encode(token_buf);

    Ok(Self(token))
  }

  pub fn value(self) -> String {
    self.0
  }
}

impl AuthSecurity for Token {
  fn hash(&self) -> AuthResult<String> {
    let hash = Sha256::digest(self.0.as_bytes());

    let hex_hash = hex::encode(hash);

    Ok(hex_hash)
  }

  fn verify(&self, hash: &str) -> AuthResult<()> {
    let token_hash = Sha256::digest(self.0.as_bytes());

    let token = hex::encode(token_hash);

    if token != *hash {
      return Err(AuthError::Invalid);
    }

    Ok(())
  }
}
