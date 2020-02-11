use crate::models::sheet::Sheet;

use actix_web::{web, Error, HttpRequest, HttpResponse, Result, http};
use std::process::Command;

pub fn create(req: HttpRequest) -> HttpResponse {
  
}

pub fn open(req: HttpRequest) -> HttpResponse {
  let config_key = req.match_info()
    .get("config_key")
    .unwrap_or("");

  match Config::get_by_key(&config_key) {
    Ok(Some(conf)) => {
      use std::path::Path;

      let folder_path = Path::new(".").join(Path::new(&conf.value));

      if cfg!(target_os = "windows") {
        println!("opening: {}", folder_path.to_str().unwrap_or("."));

        if let Err(e) = Command::new("cmd")
          .args(&["/C", "explorer", folder_path.to_str().unwrap_or(".")])
          .output() {
          println!("{}", e);

          return HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body("Internal server error: an error occured when trying to open folder");
        }
      }
      else {
        return HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("Internal server error: Target OS not supported");
      }
    },

    Ok(None) => {
      return HttpResponse::NotFound()
      .content_type("text/plain")
      .body("404 Not Found: no such config");
    }

    Err(e) => {
      println!("{}", e);

      return HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when fetching config");
    }
  }

  HttpResponse::Found()
      .header(http::header::LOCATION, "/")
      .content_type("text/plain")
      .body("created")
}