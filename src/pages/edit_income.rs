use crate::components;
use crate::models::income::Income;

use crate::utils::req_auth::request_authentication;
use crate::models::user::UserRole;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse, http};

pub async fn render(req: HttpRequest) -> HttpResponse {
  let auth_result = request_authentication(&req, UserRole::Guest);

  match auth_result {
    Ok(auth) => {
      if !auth.has_access() {
        return HttpResponse::Found()
        .header(http::header::LOCATION, "/signin")
        .content_type("text/plain")
        .body("account needed");
      }
    },
    Err(e) => {
      let view = html! {
        "an error occured when checking account informations" (e)
      };

      return HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body(view.into_string());
    }
  }

  let income_id = req
    .match_info()
    .get("income_id")
    .unwrap_or("0")
    .parse::<i32>()
    .unwrap();

  let income_result = Income::get_by_id(income_id);
  let some_income = match income_result {
    Ok(some_income) => some_income,
    Err(e) => {
      let content = html! {
        p {
          "An error occured when trying to load edit-income page" (e)
        }
      };

      let view = components::page("edit-income", &content);
      
      return HttpResponse::Ok()
      .content_type("text/html")
      .body(view.into_string());
    }
  };

  let content = html! {
    div class="title-row" {
      div class="left" {
        h1 { "Your incomes" }
      }
    }

    @match some_income {
      Some(income) => {

        div class="form-wrapper" {
          form method="post" action="/api/incomes/update-by-id" {
            h4 { "Editing income" }

            input type="hidden" name="id" value=(income_id);

            div {
              label for="name" { "Name" }
              input id="name" type="text" name="name" value=(income.name);
            }
  
            div {
              label for="amount" { "Amount" }
              input id="amount" type="number" name="amount" value=(income.amount);
            }
  
            div {
              label for="date" { "Date" }
              input id="date" type="date" name="date" value=(income.date);
            }
      
            div class="row" {
              a href={"/sheet/"(income.sheet_id)} { "cancel" }
              input type="submit" value="update";
            }
          }
        }

      },
      None => {
        div class="form-wrapper" {
          "no income with such id " span { (income_id) }
          a href="/sheets" { "go back" }
        }
      }
    }
  };

  let view = components::page("rename-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}