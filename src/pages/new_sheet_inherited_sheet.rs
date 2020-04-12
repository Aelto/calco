use crate::components;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

use crate::models::sheet::Sheet;

pub async fn render(req: HttpRequest) -> HttpResponse {
  let sheet_id = req
    .match_info()
    .get("sheet_id")
    .unwrap_or("0")
    .parse::<i32>()
    .unwrap();

  

  let content = html! {
    img src="/static/assets/undraw_setup_wizard_r6mr.svg" class="background-illustration";

    div class="title-row" {
      div class="left" {
        h1 { "Adding an income" }
      }
    }

    div class="form-wrapper" {
      form method="post" action="/api/inherited-sheets" {
        h4 { "Importing a new sheet" }
        fieldset {
          legend { "fill the information" }

          input type="hidden" name="sheet_id" value=(sheet_id);

          @match Sheet::get_all() {
            Ok(sheets) => {
              select name="inherited_sheet_id" {
                @for sheet in sheets {
                  @if sheet.id != sheet_id {
                    option value=(sheet.id) { (sheet.name) }
                  }
                }
              }
            },

            Err(e) => {
              "An error occured when loading all sheets: " (e)
            }
          }

          div {
            label for="date" { "Date" }
            input id="date" type="date" name="date";
          }
  
          div class="row" {
            a href={"/sheet/" (sheet_id)} { "cancel" }
            input type="submit" value="create";
          }
        }
      }
    }
  };

  let view = components::page("new-inherited-sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}