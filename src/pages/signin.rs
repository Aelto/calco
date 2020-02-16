use crate::components;

use maud::{html};
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {
    img class="background-illustration" src="/static/assets/undraw_authentication_fsn5.svg";

    div class="form-wrapper" {
      form method="post" action="/api/auth/signin" {
        h4 { "Sign in" }
        fieldset {
          legend { "Sign in to an account" }
  
          label for="handle" { "handle" }
          input id="handle" type="handle" name="handle";
  
          label for="password" { "password" }
          input id="password" type="password" name="password";
  
          input type="submit" value="Submit";
        }
      }
  
      div {
        "or " a href="/signup" { "sign up" } " if you don't have an account."
      }
    }
  };

  let view = components::page_without_menu("signin", &content);

  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}