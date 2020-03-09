use crate::components;
use crate::models::sheet::Sheet;
use crate::models::expense::Expense;
use crate::models::income::Income;
use crate::models::cached_sheet_value::CachedSheetValue;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::HttpResponse;

pub async fn render(req: HttpRequest) -> HttpResponse {
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
  let sheets_result = Sheet::get_all_sheets_by_parent_sheet_id(sheet_id)
    .and_then(|sheets| {
      let mut out = Vec::new();
      
      for sheet in sheets {
        let sheet_id = sheet.id;
        out.push((sheet, CachedSheetValue::get_by_sheet_id(sheet_id)?))
      }

      Ok(out)
    });


  let content = html! {
    div class="title-row" {
      div class="left" {
        h1 { (sheet.name) }
        span { "expenses and incomes" }
      }
    }

    section class="expenses-and-incomes" {

      div class="expenses" {
        div class="title-row" {
          h4 { "expenses" }
          a href={"/sheet/" (sheet_id) "/expenses/new"} {
            button { "new expense" }
          }
        }

        div class="expenses-list" {

          @match expenses_result {
            Ok(expenses) => {
              
              @for expense in expenses {
                div.expense {
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

      div class="incomes" {
        div class="title-row" {
          h4 { "incomes" }
          a href={"/sheet/" (sheet_id) "/incomes/new"} {
            button { "new income" }
          }
        }

        div class="incomes-list" {

          @match incomes_result {
            Ok(incomes) => {
              
              @for income in incomes {
                div.income {
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

      div class="sheets" {
        div class="title-row" {
          h4 { "sheets" }
          a href={"/sheet/" (sheet_id) "/inherited-sheets/new"} {
            button { "import sheet" }
          }
        }

        div class="sheets-list" {

          @match sheets_result {
            Ok(sheet_and_cached_values) => {
              
              @for (sheet, cached_sheet_value) in sheet_and_cached_values {
                div.expense {
                  div.row {
                    span.amount {
                      @match cached_sheet_value {
                        Some(cached_sheet_value) => {
                          (cached_sheet_value.value)
                        }
                        None => { "0" }
                      }
                    }
                    span.name { (sheet.name) }

                    div.actions {
                      a href={"/sheet/"(sheet.id)} { "edit" }
                      form method="post" action={"/sheet/"(sheet_id)"/sheets/delete/"(sheet.id)} {
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