use actix_web::{HttpRequest, HttpMessage};
use crate::models::user::{User, UserRole};

pub enum AuthenticationResult {
  DoesHaveAccess,
  DoesNotHaveAccess
}

impl AuthenticationResult {
  pub fn has_access(&self) -> bool {
    match *self {
      AuthenticationResult::DoesHaveAccess => true,
      AuthenticationResult::DoesNotHaveAccess => false
    }
  }
}

pub fn request_authentication(req: &HttpRequest, role: UserRole) -> Result<AuthenticationResult, String> {
  let token_cookie = req.cookie("token");

  if token_cookie.is_none() {
    return Ok(AuthenticationResult::DoesNotHaveAccess);
  }
  
  let token_cookie = token_cookie.unwrap();
  let token = token_cookie.value();
  let user = User::get_by_token(token)
    .map_err(|err| { format!("error when fetching user by token, {}", err) })?;

  if user.is_none() {
    return Ok(AuthenticationResult::DoesNotHaveAccess);
  }

  let user = user.unwrap();
  if !user.is_role_greater_or_equal(role) {
    return Ok(AuthenticationResult::DoesNotHaveAccess)
  }

  Ok(AuthenticationResult::DoesHaveAccess)
}