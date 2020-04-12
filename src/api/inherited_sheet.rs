use crate::models::inherited_sheet::InheritedSheet;
use crate::models::user::{UserRole};
use crate::models::sheet::Sheet;
use crate::utils::req_auth::request_authentication;

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};
use chrono::prelude::*;
#[derive(Serialize, Deserialize)]
pub struct CreateInheritedSheetBody {
  pub date: String,
  pub sheet_id: i32,
  pub inherited_sheet_id: i32
}

pub async fn create_inherited_sheet(req: HttpRequest, form: web::Form<CreateInheritedSheetBody>) -> Result<HttpResponse> {
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

  if let Ok(date) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") {
    let inherited_sheet = InheritedSheet::new(form.sheet_id, form.inherited_sheet_id, date.and_hms(0, 0, 0).timestamp());

    inherited_sheet.insert()
    .map_err(|err| {
      println!("error when creating inherited_sheet {}", err);
  
      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when inserting inherited_sheet into database")
    })?;

    let parent_sheet = Sheet::get_by_id(inherited_sheet.parent_sheet_id).map_err(|err| {
      println!("error when fetching parent sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when fetching parent sheet from database")
    })?;

    let child_sheet = Sheet::get_by_id(inherited_sheet.inherited_sheet_id).map_err(|err| {
      println!("error when fetching child sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when fetching child sheet from database")
    })?;

    match (parent_sheet, child_sheet) {
      (Some(mut parent_sheet), Some(child_sheet)) => {
        parent_sheet.add_to_cached_value(child_sheet.cached_value)
        .map_err(|err| {
          println!("error when updating parent sheet cached value {}", err);

          HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body("Internal server error: error when updating parent sheet cached value in database")
        })?;
      },
      _ => {}
    };
  
    Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
        .content_type("text/plain")
        .body("created")
    )
  }
  else {
    println!("error when parsing inherited_sheet date");
  
    return Ok(
      HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when parsing inherited_sheet date")
    );
  }
}
#[derive(Serialize, Deserialize)]
pub struct DeleteInheritedSheetByIdBody {
  pub sheet_id: i32,
  pub inherited_sheet_id: i32
}

pub async fn delete_inherited_sheet_by_id(req: HttpRequest, form: web::Form<DeleteInheritedSheetByIdBody>) -> Result<HttpResponse> {
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

  let some_inherited_sheet = InheritedSheet::get_by_parent_and_inherited_id(form.sheet_id, form.inherited_sheet_id)
  .map_err(|err| {
    println!("error when fetching inherited sheet {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching inherited sheet from database")
  })?;

  if let Some(inherited_sheet) = some_inherited_sheet {
    inherited_sheet.remove().map_err(|err| {
      println!("error when removing inherited sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when removing inherited sheet from database")
    })?;

    let parent_sheet = Sheet::get_by_id(inherited_sheet.parent_sheet_id).map_err(|err| {
      println!("error when fetching parent sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when fetching parent sheet from database")
    })?;

    let child_sheet = Sheet::get_by_id(inherited_sheet.inherited_sheet_id).map_err(|err| {
      println!("error when fetching child sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when fetching child sheet from database")
    })?;

    match (parent_sheet, child_sheet) {
      (Some(mut parent_sheet), Some(child_sheet)) => {
        parent_sheet.remove_from_cached_value(child_sheet.cached_value)
        .map_err(|err| {
          println!("error when updating parent sheet cached value {}", err);

          HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body("Internal server error: error when updating parent sheet cached value in database")
        })?;
      },
      _ => {}
    };
  }

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
      .content_type("text/plain")
      .body("created")
  )
}