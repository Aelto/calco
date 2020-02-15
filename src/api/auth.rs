use crate::models::invitation::Invitation;
use actix_web::{http, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::models::user::{User, set_user_token};
use crate::utils::crypto::{verify_passwords};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SignupBody {
  pub handle: String,
  pub password: String,
  pub passwordconfirm: String,
  pub invitation_hash: String,
}

pub async fn signup(req: web::Form<SignupBody>) -> Result<HttpResponse> {
  if req.password.len() == 0 || req.handle.len() == 0 || req.invitation_hash.len() == 0 {
    return Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("handle, password and invitation hash are required"),
    );
  }

  if req.password != req.passwordconfirm {
    return Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("passwords do not match"),
    );
  }

  let invitation = Invitation::get_by_hash(&req.invitation_hash)
    .map_err(|err| {
      println!("error when fetching invitation by hash {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: could not fetch invitation from db")
    })
    .and_then(|invitation| match invitation {
      Some(inv) => Ok(inv),
      None => 
        Err(HttpResponse::Unauthorized()
          .content_type("text/plain")
          .body("no such invitation"))
    })?;

    if invitation.handle != req.handle {
      return Ok(
        HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("no such invitation")
      );
    }
    
    invitation.consume(req.password.clone())
    .map_err(|err| {
      println!("error on invitation.consume {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: could not use the invitation")
    })?;

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/signin")
      .finish()
      .into_body()
  )
}

#[derive(Serialize, Deserialize)]
pub struct SigninBody {
  pub handle: String,
  pub password: String
}

pub async fn signin(req: web::Form<SigninBody>) -> Result<HttpResponse> {
  let user = User::get_by_handle(&req.handle)
    .map_err(|err| {
      println!("error when fetching user by handle {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: could not fetch user from db")
    })?;

  if let Some(user) = user {
    if !verify_passwords(&req.handle, &req.password, &user.password) {
      return Ok(
        HttpResponse::Unauthorized()
          .content_type("text/plain")
          .body("user & password do no match")
      );
    }

    let token = Uuid::new_v4().to_string();

    set_user_token(&req.handle, &token)
      .map_err(|err| {
        println!("error when updating user token {}, {}", req.handle, err);

        HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body("Internal server error: error when updating user token")
      })?;

    let cookie = actix_web::cookie::Cookie::build("token", token)
      .path("/")
      // .secure(true)
      .finish();

    Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .header(http::header::SET_COOKIE, cookie.to_string())
        .finish()
        .into_body()
    )
  }
  else {
    Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("user & password do no match")
    )
  }
}