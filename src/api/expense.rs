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
pub struct UpdateExpenseByIdBody {
  pub name: String,
  pub amount: i32,
  pub date: String,
  pub id: i32
}

pub async fn update_expense_by_id(req: HttpRequest, form: web::Form<UpdateExpenseByIdBody>) -> Result<HttpResponse> {
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

  let some_expense = Expense::get_by_id(form.id).map_err(|err| {
    println!("error when fetching expense {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching expense from database")
  })?;

  if let Some(mut expense) = some_expense {
    expense.amount = form.amount;
    expense.name = form.name.clone();

    if let Ok(date) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") {
      expense.date = date.and_hms(0, 0, 0).timestamp();
    }  

    expense.update().map_err(|err| {
      println!("error when updating expense {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when renaming sheet from database")
    })?;

    return Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, format!("/sheet/{}", expense.sheet_id))
        .content_type("text/plain")
        .body("update")
    );
  }

  return Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, format!("/expenses/{}", form.id))
      .content_type("text/plain")
      .body("update")
  );
}

#[derive(Serialize, Deserialize)]
pub struct DeleteExpenseByIdBody {
  pub id: i32,
  pub sheet_id: i32
}

pub async fn delete_sheet_by_id(req: HttpRequest, form: web::Form<DeleteExpenseByIdBody>) -> Result<HttpResponse> {
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

  let some_expense = Expense::get_by_id(form.id).map_err(|err| {
    println!("error when fetching expense {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching expense from database")
  })?;

  if let Some(expense) = some_expense {
    expense.remove().map_err(|err| {
      println!("error when removing expense {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when removing expense from database")
    })?;
  }

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
      .content_type("text/plain")
      .body("created")
  )
}