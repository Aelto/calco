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

  let content = html! {
    img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      div class="left" {
        h1 { "Your sheets" }
        // span { "creating a new sheet" }
      }
    }

    div class="form-wrapper" {
      form method="post" action="/api/sheets" {
        h4 { "Creating a new sheet" }
        fieldset {
          legend { "fill the information" }
          div {
            label for="name" { "Name" }
            input id="name" type="text" name="name";
          }
  
          div class="row" {
            a href="/sheets" { "cancel" }
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