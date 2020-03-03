#![feature(proc_macro_hygiene)]

extern crate chrono;

use actix_web::{App, web, HttpServer};
use actix_files as fs;

mod pages;
mod components;
mod constants;
mod models;
mod api;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  match models::create_database() {
    Ok(()) => println!("database ready"),
    Err(e) => panic!(e)
  };

  let port: i32 = std::env::args()
    .nth(1)
    .and_then(|n| n.parse::<i32>().ok())
    .unwrap_or(5000);

  println!("starting server on port {}", port);

  HttpServer::new(|| {
    App::new()
    // home page
    .service(web::resource("/").route(web::get().to(pages::root::render)))

    // auth pages
    .route("/signup", web::get().to(pages::signup::render))
    .route("/signin", web::get().to(pages::signin::render))
    
    // sheets pages
    .service(web::resource("/sheets").route(web::get().to(pages::sheets::render)))
    .service(web::resource("/new-sheet").route(web::get().to(pages::new_sheet::render)))
    .service(web::resource("/sheet/rename/{sheet_id}").route(web::get().to(pages::rename_sheet::render)))
    .service(web::resource("/sheet/{sheet_id}").route(web::get().to(pages::sheet::render)))
    .service(web::resource("/sheet/{sheet_id}/expenses/new").route(web::get().to(pages::new_sheet_expense::render)))
    .service(web::resource("/expense/{expense_id}/edit").route(web::get().to(pages::edit_expense::render)))

    // static files
    .service(fs::Files::new("/static", "./static"))

    // api endpoints
    .service(
      web::scope("/api")
        .route("/auth/signup", web::post().to(api::auth::signup))
        .route("/auth/signin", web::post().to(api::auth::signin))
        .route("/users/delete-by-id", web::post().to(api::users::delete_user))
        .route("/sheets", web::post().to(api::sheet::create_sheet))
        .route("/sheets/delete-by-id", web::post().to(api::sheet::delete_sheet_by_id))
        .route("/sheets/rename-by-id", web::post().to(api::sheet::rename_sheet_by_id))
        .route("/expenses", web::post().to(api::expense::create_expense))
        .route("/expenses/delete-by-id", web::post().to(api::expense::delete_sheet_by_id))
        .route("/expenses/update-by-id", web::post().to(api::expense::update_expense_by_id))
        .route("/invitations", web::post().to(api::invitations::create_invitation))
    )

  })
  .bind(format!("127.0.0.1:{}", port))?
  .run()
  .await
}