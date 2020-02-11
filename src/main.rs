#![feature(proc_macro_hygiene)]

use actix_web::{web, App, HttpServer};
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

  use actix_web::{web, Error, HttpRequest, HttpResponse, Result, http,};

  HttpServer::new(|| {
    App::new()
    .service(web::resource("/").route(web::get().to(pages::root::render)))
    .service(fs::Files::new("/static", "./static"))
    // .service(
    //   web::scope("/api")
    //   .route("/explorer/open/{config_key}", web::get().to(api::explorer::open))
    //   .service(
    //     web::resource("/test").to(|req: HttpRequest| match *req.method() {
    //       http::Method::GET => HttpResponse::Ok(),
    //       http::Method::POST => HttpResponse::MethodNotAllowed(),
    //       _ => HttpResponse::NotFound(),
    //   }),
    //   )
    // )
  })
  .bind(format!("127.0.0.1:{}", port))?
  .run()
  .await
}