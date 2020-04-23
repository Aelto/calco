use crate::components;
use crate::models::sheet::Sheet;

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

  let sheet_result = Sheet::get_by_id(sheet_id);
  let some_sheet = match sheet_result {
    Ok(some_sheet) => some_sheet,
    Err(e) => {
      let content = html! {
        p {
          "An error occured when trying to load sheet-rename page" (e)
        }
      };

      let view = components::page("rename-sheet", &content);
      
      return HttpResponse::Ok()
      .content_type("text/html")
      .body(view.into_string());
    }
  };

  let content = html! {
    div class="title-row" {
      div class="left" {
        h1 { "Your sheets" }
      }
    }

    @match some_sheet {
      Some(sheet) => {

        div class="form-wrapper" {
          form method="post" action="/api/sheets/rename-by-id" {
            h4 { "Renaming sheet" }
            // fieldset {
              // legend { "enter a new name" }

              
            // }

            input type="hidden" name="id" value=(sheet_id);

              div {
                label for="name" { "Name" }
                input id="name" type="text" name="name" value=(sheet.name);
              }
      
              div class="row" {
                a href={"/sheet/"(sheet_id)} { "cancel" }
                input type="submit" value="rename";
              }
          }
        }

      },
      None => {
        div class="form-wrapper" {
          "no sheet with such id " span { (sheet_id) }
          a href={"/sheet/"(sheet_id)} { "go back" }
        }
      }
    }
  };

  let view = components::page("rename-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}