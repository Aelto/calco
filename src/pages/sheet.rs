use crate::components;
use crate::models::sheet::Sheet;
use crate::models::expense::Expense;
use crate::models::income::Income;

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

  let sheet_id = req
    .match_info()
    .get("sheet_id")
    .unwrap_or("0")
    .parse::<i32>()
    .unwrap();

  let sheet_result = Sheet::get_by_id(sheet_id);
  let some_sheet = match sheet_result {
    Ok(some_sheet) => some_sheet,
    Err(e) => {
      let content = html! {
        p {
          "An error occured when trying to load sheet page" (e)
        }
      };

      let view = components::page("sheet", &content);
      
      return HttpResponse::Ok()
      .content_type("text/html")
      .body(view.into_string());
    }
  };


  if some_sheet.is_none() {
    let content = html! {
      div class="form-wrapper" {
        "no sheet with such id " span { (sheet_id) }
        a href="/sheets" { "go back" }
      }
    };

    let view = components::page("sheet", &content);
    
    return HttpResponse::Ok()
    .content_type("text/html")
    .body(view.into_string());
  }

  let sheet = some_sheet.unwrap();
  let expenses_result = Expense::get_all_by_sheet_id(sheet.id);
  let incomes_result = Income::get_all_by_sheet_id(sheet.id);
  let sheets_result = Sheet::get_all_sheets_by_parent_sheet_id(sheet_id);


  let content = html! {
    div class="title-row" {
      div {
        h1 { (sheet.name) }
        a href={"/sheet/rename/" (sheet_id)} { "rename" }
      
        form method="post" action="/api/sheets/delete-by-id" {
          input type="hidden" name="id" value=(sheet_id);
          input type="submit" value="delete" class="link";
        }
      }
      div {
        "expenses and incomes, currently at " (sheet.cached_value) span {"EUR"}
      }
    }

    section class="expenses-and-incomes" {

      div class="expenses column" {
        div class="title-row" {
          h4 { "expenses" }
          a href={"/sheet/" (sheet_id) "/expenses/new"} class="button" {
            "new expense" span{"+"}
          }
        }

        div class="expenses-list" {

          @match expenses_result {
            Ok(expenses) => {
              
              @for expense in expenses {
                div.expense.element {
                  div.row {
                    span.amount { (expense.amount) }
                    span.name { (expense.name) }

                    div.actions {
                      a href={"/expense/"(expense.id)"/edit"} { "edit" }
                      form method="post" action="/api/expenses/delete-by-id" {
                        input type="hidden" name="id" value=(expense.id);
                        input type="hidden" name="sheet_id" value=(sheet_id);
                        input.link type="submit" value="delete";
                      }
                    }
                  }
                }
              }

            },

            Err(e) => {
              "error fetching expenses " (e)
            }
          }

        }
      }

      div class="incomes column" {
        div class="title-row" {
          h4 { "incomes" }
          a href={"/sheet/" (sheet_id) "/incomes/new"} class="button" {
            "new income" span{"+"}
          }
        }

        div class="incomes-list" {

          @match incomes_result {
            Ok(incomes) => {
              
              @for income in incomes {
                div.income.element {
                  div.row {
                    span.amount { (income.amount) }
                    span.name { (income.name) }

                    div.actions {
                      a href={"/income/"(income.id)"/edit"} { "edit" }
                      form method="post" action="/api/incomes/delete-by-id" {
                        input type="hidden" name="id" value=(income.id);
                        input type="hidden" name="sheet_id" value=(sheet_id);
                        input.link type="submit" value="delete";
                      }
                    }
                  }
                }
              }

            },

            Err(e) => {
              "error fetching incomes " (e)
            }
          }

        }
      }

      div class="sheets column" {
        div class="title-row" {
          h4 { "sheets" }
          a href={"/sheet/" (sheet_id) "/inherited-sheets/new"} class="button" {
            "import sheet" span{"+"}
          }
        }

        div class="sheets-list" {

          @match sheets_result {
            Ok(sheets) => {
              
              @for sheet in sheets {
                div.sheet.element {
                  div.row {
                    span.amount { (sheet.cached_value) }
                    span.name { (sheet.name) }

                    div.actions {
                      a href={"/sheet/"(sheet.id)} { "edit" }
                      form method="post" action={"/api/inherited-sheets/delete"} {
                        input type="hidden" name="sheet_id" value=(sheet_id);
                        input type="hidden" name="inherited_sheet_id" value=(sheet.id);

                        input.link type="submit" value="delete";
                      }
                    }
                  }
                }
              }

            },

            Err(e) => {
              "error fetching sheets " (e)
            }
          }

        }
      }

    }
  };

  let view = components::page("sheet", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}