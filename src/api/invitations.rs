use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::models::user::UserRole;
use crate::utils::req_auth::{request_authentication};
use crate::models::invitation::{Invitation, update_invitation_expiration};

#[derive(Serialize, Deserialize)]
pub struct CreateInvitationBody {
  pub handle: String,
  pub role: i32,
}

pub async fn create_invitation(req: HttpRequest, form: web::Form<CreateInvitationBody>) -> Result<HttpResponse> {
  let auth_result = request_authentication(&req, UserRole::Admin)
    .map_err(|err| {
      println!("error");
      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body(err)
    })?;

  if !auth_result.has_access() {
    return Ok(
      HttpResponse::NotFound()
        .content_type("text/plain")
        .body("HTTP 404: Not found")
    ); 
  }

  let invitation = if let Some(inv) = Invitation::get_by_handle(&form.handle)
    .map_err(|_err| {
      println!("error when checking if invitation already exists");

      HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when checking if invitation already exists")
    })? {
    
    update_invitation_expiration(&form.handle)
      .map_err(|_err| {
        println!("error when updating already existing invitation");

        HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when updating already existing invitation")
      })?;

    inv
  }
  else {
    let invitation = Invitation::new(&form.handle, UserRole::from_number(form.role));

    Invitation::insert(&invitation)
    .map_err(|_err| {
      println!("error when inserting new invitation");

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: invitation insertion failed")
    })?;

    invitation
  };

  Ok(
    HttpResponse::Found()
      .content_type("text/plain")
      .body(format!("created: {}", invitation.to_url()))
  )
}