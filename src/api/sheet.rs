use crate::models::sheet::Sheet;
use crate::models::user::{UserRole};
use crate::utils::req_auth::request_authentication;

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};

#[derive(Serialize, Deserialize)]
pub struct CreateSheetBody {
  pub name: String,
}

pub fn create_sheet(req: HttpRequest, form: web::Form<CreateSheetBody>) -> Result<HttpResponse> {
  let auth_result = request_authentication(&req, UserRole::Guest);

  match auth_result {
    Ok(auth) => {
      if !auth.has_access() {
        return Ok(
          HttpResponse::NotFound()
            .content_type("text/plain")
            .body("HTTP 404: Not found"),
        );
      }
    }
    Err(e) => {
      return Ok(
        HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body(e),
      )
    }
  };

  if form.name.len() == 0 {
    return Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("sheet name is required")
    );
  }

  let sheet = Sheet::new(&form.name);
  
  sheet.insert()
  .map_err(|err| {
    println!("error when creating sheet {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when inserting sheet into database")
  })?;

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/sheets")
      .content_type("text/plain")
      .body("created")
  )
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSheetBody {
  pub name: String,
}

pub fn delete_sheet_by_name(req: HttpRequest, form: web::Form<CreateSheetBody>) -> Result<HttpResponse> {
  let auth_result = request_authentication(&req, UserRole::Guest);

  match auth_result {
    Ok(auth) => {
      if !auth.has_access() {
        return Ok(
          HttpResponse::NotFound()
            .content_type("text/plain")
            .body("HTTP 404: Not found"),
        );
      }
    }
    Err(e) => {
      return Ok(
        HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body(e),
      )
    }
  };

  if form.name.len() == 0 {
    return Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("sheet name is required")
    );
  }

  let some_sheet = Sheet::get_by_name(&form.name).map_err(|err| {
    println!("error when fetching sheet {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching sheet from database")
  })?;

  if let Some(sheet) = some_sheet {
    sheet.remove().map_err(|err| {
      println!("error when removing sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when removing sheet from database")
    })?;
  }

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/sheets")
      .content_type("text/plain")
      .body("created")
  )
}