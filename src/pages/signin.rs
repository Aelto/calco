use crate::components;

use maud::{html};
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {
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
  };

  let view = components::page("root", &content);

  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}