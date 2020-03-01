use crate::models::sheet::Sheet;
use crate::models::expense::Expense;
use crate::models::user::{UserRole};
use crate::utils::req_auth::request_authentication;

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};
use chrono::prelude::*;
#[derive(Serialize, Deserialize)]
pub struct CreateExpenseBody {
  pub name: String,
  pub amount: i32,
  pub date: String,
  pub sheet_id: i32
}

pub async fn create_expense(req: HttpRequest, form: web::Form<CreateExpenseBody>) -> Result<HttpResponse> {
  let auth_result = request_authentication(&req, UserRole::Guest);

  println!("test");

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

  println!("{}", form.date);

  if let Err(e) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") {
    println!("{}", e);
  }

  if let Ok(date) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") {
    let expense = Expense::new(&form.name, form.amount, form.sheet_id, date.and_hms(0, 0, 0).timestamp());

    expense.insert()
    .map_err(|err| {
      println!("error when creating expense {}", err);
  
      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when inserting expense into database")
    })?;
  
    Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
        .content_type("text/plain")
        .body("created")
    )
  }
  else {
    println!("error when parsing expense date");
  
    return Ok(
      HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when parsing expense date")
    );
  }
}

#[derive(Serialize, Deserialize)]
pub struct RenameSheetByIdBody {
  pub name: String,
  pub id: i32
}

pub async fn rename_sheet_by_id(req: HttpRequest, form: web::Form<RenameSheetByIdBody>) -> Result<HttpResponse> {
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

  let some_sheet = Sheet::get_by_id(form.id).map_err(|err| {
    println!("error when fetching sheet {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching sheet from database")
  })?;

  if let Some(mut sheet) = some_sheet {
    sheet.name = form.name.clone();

    sheet.update().map_err(|err| {
      println!("error when removing sheet {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when renaming sheet from database")
    })?;
  }

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/sheets")
      .content_type("text/plain")
      .body("renamed")
  )
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSheetByIdBody {
  pub id: i32,
}

pub async fn delete_sheet_by_id(req: HttpRequest, form: web::Form<DeleteSheetByIdBody>) -> Result<HttpResponse> {
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

  let some_sheet = Sheet::get_by_id(form.id).map_err(|err| {
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