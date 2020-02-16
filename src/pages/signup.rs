use crate::components;
use maud::{html, DOCTYPE};
use actix_web::web;
use serde::Deserialize;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;


#[derive(Deserialize)]
pub struct Info {
  handle: String,
  hash: String
}

pub async fn render(info: Option<web::Query<Info>>, req: HttpRequest) -> HttpResponse {
  let content = html! {
    form method="post" action="/api/auth/signup" {
      h4 { "Signup" }
      fieldset {
        legend { "Create an account" }

        @match info {
          Some(params) => {
            label for="invitation_hash" { "invitation token" }
            input id="invitation_hash" type="password" name="invitation_hash" value=(params.hash) readonly="true";

            label for="handle" { "handle" }
            input id="handle" type="text" name="handle" value=(params.handle) readonly="true";
          },
          None => {
            label for="handle" { "handle" }
            input id="handle" type="text" name="handle";
          }
        }

        label for="password" { "password" }
        input id="password" type="password" name="password";

        label for="passwordconfirm" { "confirm password" }
        input id="passwordconfirm" name="passwordconfirm" type="password";

        input type="submit" value="Submit";
      }
    }

    p {
      "You need an invitation to create an account, ask an administrator for one."
    }

    p {
      "You can " a href="/signin" { "signin" } " if you already have an account."
    }
  };

  let view = components::page_without_menu("root", &content);

  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}