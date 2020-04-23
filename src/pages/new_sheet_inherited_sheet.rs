use crate::components;

use crate::utils::req_auth::request_authentication;
use crate::models::user::UserRole;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse, http};

use crate::models::sheet::Sheet;

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
        h1 { "Adding an income" }
      }
    }

    div class="form-wrapper" {
      form method="post" action="/api/inherited-sheets" {
        h4 { "Importing a new sheet" }
        fieldset {
          legend { "fill the information" }

          input type="hidden" name="sheet_id" value=(sheet_id);

          @match Sheet::get_all() {
            Ok(sheets) => {
              select name="inherited_sheet_id" {
                @for sheet in sheets {
                  @if sheet.id != sheet_id {
                    option value=(sheet.id) { (sheet.name) }
                  }
                }
              }
            },

            Err(e) => {
              "An error occured when loading all sheets: " (e)
            }
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

  let view = components::page("new-inherited-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}