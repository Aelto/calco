use crate::components;

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

  let sheet_id = req
    .match_info()
    .get("sheet_id")
    .unwrap_or("0")
    .parse::<i32>()
    .unwrap();

  let content = html! {
    img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      div class="left" {
        h1 { "Adding an expense" }
      }
    }

    div class="form-wrapper" {
      form method="post" action="/api/expenses" {
        h4 { "Creating a new expense" }
        fieldset {
          legend { "fill the information" }

          input type="hidden" name="sheet_id" value=(sheet_id);

          div {
            label for="name" { "Name" }
            input id="name" type="text" name="name";
          }

          div {
            label for="amount" { "Amount" }
            input id="amount" type="number" name="amount";
          }

          div {
            label for="date" { "Date" }
            input id="date" type="date" name="date";
          }
  
          div class="row" {
            a href={"/sheet/" (sheet_id)} { "cancel" }
            input type="submit" value="create";
          }
        }
      }
    }
  };

  let view = components::page("new-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}