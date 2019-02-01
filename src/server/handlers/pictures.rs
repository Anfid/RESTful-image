use crate::db::Database;
use crate::models::{Picture, PictureNew};
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json;

/// Handler for POST /pictures
pub fn post(req: &HttpRequest<Database>) -> impl Responder {
    if !req.headers().contains_key("content-type") {
        return HttpResponse::BadRequest()
            .body("Requests with unspecified Content-Type are not supported");
    }

    let content_type = req
        .headers()
        .get("content-type")
        .unwrap() // safe to unwrap, checked above
        .to_str()
        .expect("Unable to parse header content"); // TODO

    if content_type.starts_with("multipart/form-data") {
        log::info!("multipart/form-data");
    } else if content_type.starts_with("application/json") {
        log::info!("application/json");
    } else {
        return HttpResponse::BadRequest().body("Unsupported Content-Type");
    }

    HttpResponse::Created().json(save(
        req.state(),
        PictureNew {
            name: "1",
            image: "2",
            description: None,
        },
    ))
}

fn save(db: &Database, pic: PictureNew) -> Result<Picture, String> {
    match db.insert(pic) {
        Ok(pic) => Ok(pic),
        Err(e) => {
            log::error!("{}", e);
            Err("Error saving file".to_owned())
        }
    }
}
