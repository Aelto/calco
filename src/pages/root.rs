use crate::components;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {

  };
  let view = components::page("root", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}