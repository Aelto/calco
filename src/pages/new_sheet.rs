use crate::components;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {
    img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      div class="left" {
        h1 { "Your sheets" }
        span { "creating a new sheet" }
      }
    }

    div class="form-wrapper" {
      form method="post" action="/api/sheet" {
        div {
          label for="name" { "Name" }
          input id="name" type="text";
        }

        input type="submit" value="create";
      }
    }
  };

  let view = components::page("new-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}