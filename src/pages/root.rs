use crate::components;
use crate::utils::req_auth::request_authentication;
use crate::models::user::UserRole;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(req: HttpRequest) -> HttpResponse {
  let auth_result = request_authentication(&req, UserRole::Guest);

  let content = html! {
    @match auth_result {
      Ok(auth) => {
        @if auth.has_access() {

        }
        else {
          div {
            "You are not signed-in, please "
            a href="/signin" { "sign-in" } " or "
            a href="/signup" { "sign-up" } " if you don't have an account"
          }
        }
      }
      Err(e) => {
        "an error occured when checking account informations" (e)
      }
    }
  };
  let view = components::page("root", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}