#![feature(proc_macro_hygiene)]

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
    .service(web::resource("/").route(web::get().to(pages::root::render)))
    .route("/signup", web::get().to(pages::signup::render))
    .route("/signin", web::get().to(pages::signin::render))
    
    .service(web::resource("/sheets").route(web::get().to(pages::sheets::render)))
    .service(web::resource("/new-sheet").route(web::get().to(pages::new_sheet::render)))
    .service(fs::Files::new("/static", "./static"))
    .service(
      web::scope("/api")
        .route("/auth/signup", web::post().to(api::auth::signup))
        .route("/auth/signin", web::post().to(api::auth::signin))
        .route("/users/delete-by-id", web::post().to(api::users::delete_user))
        .route("/invitations", web::post().to(api::invitations::create_invitation))
    )
  })
  .bind(format!("127.0.0.1:{}", port))?
  .run()
  .await
}