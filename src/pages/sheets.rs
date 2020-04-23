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

  let sheets_result = Sheet::get_all();

  let content = html! {
    // img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      h1 { "Your sheets" }
      a href="/new-sheet" class="button" { "new sheet" span { "+" } }
    }


    @match sheets_result {
      Ok(sheets) => {
        ul class="sheets" {
          @for sheet in &sheets {
            
            div class="sheet" {
              a href={"/sheet/" (sheet.id)} class="name" { (sheet.name) }
              div { (sheet.cached_value) span{"EUR"} }

              // div class="bottom-row" {
              //   div class="actions" {
              //     a href={"/sheet/rename/" (sheet.id)} { "rename" }

              //     form method="post" action="/api/sheets/delete-by-id" {
              //       input type="hidden" name="id" value=(sheet.id);
              //       input type="submit" value="delete" class="link";
              //     }
              //   }
              // }
            }
          }
          
        }
      },

      Err(e) => {
        "an error occured when fetching sheets list " (e) 
      }
    }
  };

  let view = components::page("sheets", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}