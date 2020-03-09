use crate::models::income::Income;
use crate::models::user::{UserRole};
use crate::utils::req_auth::request_authentication;

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};
use chrono::prelude::*;
#[derive(Serialize, Deserialize)]
pub struct CreateIncomeBody {
  pub name: String,
  pub amount: i32,
  pub date: String,
  pub sheet_id: i32
}

pub async fn create_income(req: HttpRequest, form: web::Form<CreateIncomeBody>) -> Result<HttpResponse> {
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
    let income = Income::new(&form.name, form.amount, form.sheet_id, date.and_hms(0, 0, 0).timestamp());

    income.insert()
    .map_err(|err| {
      println!("error when creating income {}", err);
  
      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when inserting income into database")
    })?;
  
    Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
        .content_type("text/plain")
        .body("created")
    )
  }
  else {
    println!("error when parsing income date");
  
    return Ok(
      HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when parsing income date")
    );
  }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateincomeByIdBody {
  pub name: String,
  pub amount: i32,
  pub date: String,
  pub id: i32
}

pub async fn update_income_by_id(req: HttpRequest, form: web::Form<UpdateincomeByIdBody>) -> Result<HttpResponse> {
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

  let some_income = Income::get_by_id(form.id).map_err(|err| {
    println!("error when fetching income {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching income from database")
  })?;

  if let Some(mut income) = some_income {
    income.amount = form.amount;
    income.name = form.name.clone();

    if let Ok(date) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") {
      income.date = date.and_hms(0, 0, 0).timestamp();
    }  

    income.update().map_err(|err| {
      println!("error when updating income {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when renaming sheet from database")
    })?;

    return Ok(
      HttpResponse::Found()
        .header(http::header::LOCATION, format!("/sheet/{}", income.sheet_id))
        .content_type("text/plain")
        .body("update")
    );
  }

  return Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, format!("/incomes/{}", form.id))
      .content_type("text/plain")
      .body("update")
  );
}

#[derive(Serialize, Deserialize)]
pub struct DeleteincomeByIdBody {
  pub id: i32,
  pub sheet_id: i32
}

pub async fn delete_income_by_id(req: HttpRequest, form: web::Form<DeleteincomeByIdBody>) -> Result<HttpResponse> {
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

  let some_income = Income::get_by_id(form.id).map_err(|err| {
    println!("error when fetching income {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when searching income from database")
  })?;

  if let Some(income) = some_income {
    income.remove().map_err(|err| {
      println!("error when removing income {}", err);

      HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: error when removing income from database")
    })?;
  }

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, format!("/sheet/{}", form.sheet_id))
      .content_type("text/plain")
      .body("created")
  )
}