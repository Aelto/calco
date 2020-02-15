use crate::models::sheet::Sheet;
use crate::models::user::{UserRole};
use crate::utils::req_auth::request_authentication;

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};

#[derive(Serialize, Deserialize)]
pub struct CreateSheetBody {
  pub name: String,
}

pub fn create_sheet(req: HttpRequest, form: web::Form<CreateSheetBody>) -> Result<HttpResponse> {
  let auth_result = request_authentication(&req, UserRole::Guest);

  match auth_result {
    Ok(auth) => {
      if !auth.has_access() {
        return Ok(
          HttpResponse::NotFound()
            .content_type("text/plain")
            .body("HTTP 404: Not found"),
        );
      }
    }
    Err(e) => {
      return Ok(
        HttpResponse::InternalServerError()
          .content_type("text/plain")
          .body(e),
      )
    }
  };

  if form.name.len() == 0 {
    return Ok(
      HttpResponse::Unauthorized()
        .content_type("text/plain")
        .body("sheet name is required")
    );
  }

  let sheet = Sheet::new(&form.name);
  
  sheet.insert()
  .map_err(|err| {
    println!("error when creating sheet {}", err);

    HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body("Internal server error: error when inserting sheet into database")
  })?;

  Ok(
    HttpResponse::Found()
      .header(http::header::LOCATION, "/sheets")
      .content_type("text/plain")
      .body("created")
  )
}

// pub fn open(req: HttpRequest) -> HttpResponse {
//   let config_key = req.match_info()
//     .get("config_key")
//     .unwrap_or("");

//   match Config::get_by_key(&config_key) {
//     Ok(Some(conf)) => {
//       use std::path::Path;

//       let folder_path = Path::new(".").join(Path::new(&conf.value));

//       if cfg!(target_os = "windows") {
//         println!("opening: {}", folder_path.to_str().unwrap_or("."));

//         if let Err(e) = Command::new("cmd")
//           .args(&["/C", "explorer", folder_path.to_str().unwrap_or(".")])
//           .output() {
//           println!("{}", e);

//           return HttpResponse::InternalServerError()
//           .content_type("text/plain")
//           .body("Internal server error: an error occured when trying to open folder");
//         }
//       }
//       else {
//         return HttpResponse::InternalServerError()
//         .content_type("text/plain")
//         .body("Internal server error: Target OS not supported");
//       }
//     },

//     Ok(None) => {
//       return HttpResponse::NotFound()
//       .content_type("text/plain")
//       .body("404 Not Found: no such config");
//     }

//     Err(e) => {
//       println!("{}", e);

//       return HttpResponse::InternalServerError()
//       .content_type("text/plain")
//       .body("Internal server error: error when fetching config");
//     }
//   }

//   HttpResponse::Found()
//       .header(http::header::LOCATION, "/")
//       .content_type("text/plain")
//       .body("created")
// }