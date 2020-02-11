use crate::constants::SALT_COMPONENT;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

pub fn hash_password(handle: &str, password: &str) -> Result<String, std::string::FromUtf8Error> {
  let salted_handle = salt(handle);
  let mut to_store: Credential = [0u8; CREDENTIAL_LEN];

  pbkdf2::derive(
    PBKDF2_ALG,
    NonZeroU32::new(100_00).unwrap(),
    &salted_handle,
    password.as_bytes(),
    &mut to_store
  );

  Ok(String::from_utf8_lossy(&to_store).replace(char::from(0), ""))
}

pub fn verify_passwords(handle: &str, unhashed_attempt: &str, actual_password: &str) -> bool {
  match hash_password(handle, unhashed_attempt) {
    Ok(p) => p == actual_password,
    Err(_err) => false
  }
}

pub fn salt(handle: &str) -> Vec<u8> {
  let mut salt = Vec::with_capacity(
    SALT_COMPONENT.len() + handle.as_bytes().len()
  );

  salt.extend(SALT_COMPONENT.as_ref());
  salt.extend(handle.as_bytes());

  salt
}