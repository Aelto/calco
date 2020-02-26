use crate::components;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {
    // img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      div class="left" {
        h1 { "Your sheets" }
        span { "viewing your sheets" }
      }
      a href="/new-sheet" {
        button { "create a sheet" }
      }
    }


    ul {
      li {}
    }
  };

  let view = components::page("sheets", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}