use crate::components;
use crate::models::expense::Expense;

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

  let expense_id = req
    .match_info()
    .get("expense_id")
    .unwrap_or("0")
    .parse::<i32>()
    .unwrap();

  let expense_result = Expense::get_by_id(expense_id);
  let some_expense = match expense_result {
    Ok(some_expense) => some_expense,
    Err(e) => {
      let content = html! {
        p {
          "An error occured when trying to load edit-expense page" (e)
        }
      };

      let view = components::page("edit-expense", &content);
      
      return HttpResponse::Ok()
      .content_type("text/html")
      .body(view.into_string());
    }
  };

  let content = html! {
    div class="title-row" {
      div class="left" {
        h1 { "Your expenses" }
      }
    }

    @match some_expense {
      Some(expense) => {

        div class="form-wrapper" {
          form method="post" action="/api/expenses/update-by-id" {
            h4 { "Editing expense" }

            input type="hidden" name="id" value=(expense_id);

            div {
              label for="name" { "Name" }
              input id="name" type="text" name="name" value=(expense.name);
            }
  
            div {
              label for="amount" { "Amount" }
              input id="amount" type="number" name="amount" value=(expense.amount);
            }
  
            div {
              label for="date" { "Date" }
              input id="date" type="date" name="date" value=(expense.date);
            }
      
            div class="row" {
              a href={"/sheet/"(expense.sheet_id)} { "cancel" }
              input type="submit" value="update";
            }
          }
        }

      },
      None => {
        div class="form-wrapper" {
          "no expense with such id " span { (expense_id) }
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