use actix_web::{http, web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::models::user::{UserRole, delete_user_by_id};
use crate::utils::req_auth::request_authentication;

#[derive(Serialize, Deserialize)]
pub struct DeleteUserBody {
  pub id: i32
}

pub async fn delete_user(req: HttpRequest, form: web::Form<DeleteUserBody>) -> Result<HttpResponse> {
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

  delete_user_by_id(form.id)
  .map_err(|err| {
    println!("error when deleting user by id {}", err);

    HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: an error occured when deleting the user")
  })?;

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/private/users")
      .content_type("text/plain")
      .body("deleted")
  )
}