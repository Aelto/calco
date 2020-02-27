use crate::components;
use crate::models::sheet::Sheet;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let sheets_result = Sheet::get_all();

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


    @match sheets_result {
      Ok(sheets) => {
        section class="sheets" {
          @for sheet in &sheets {
            div class="sheet" {
              div class="name" { (sheet.name) }

              div class="bottom-row" {
                div class="actions" {
                  a href={"/sheet/rename/" (sheet.id)} { "rename" }

                  form method="post" action="/api/sheets/delete-by-id" {
                    input type="hidden" name="id" value=(sheet.id);
                    input type="submit" value="delete" class="link";
                  }
                }
              }

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