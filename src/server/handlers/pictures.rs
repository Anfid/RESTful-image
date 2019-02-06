use crate::db::Database;
use crate::models::{Picture, PictureNew};
use actix_web::{
    dev, error, multipart::MultipartItem, Error, FutureResponse, HttpMessage, HttpRequest,
    HttpResponse, Responder,
};
use futures::{future::Future, Stream};
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
        Box::new(
            req.multipart()
                .map_err(actix_web::error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten()
                .collect()
                .map(|sizes| HttpResponse::Ok().json(sizes))
                .map_err(|e| {
                    println!("Failed: {}", e);
                    e
                }),
        )
    } else if content_type.starts_with("application/json") {
        log::info!("application/json");
        unimplemented!()
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

fn handle_multipart_item(
    item: MultipartItem<dev::Payload>,
) -> Box<Stream<Item = i64, Error = Error>> {
    log::info!("Handling multipart item");
    match item {
        MultipartItem::Field(f) => {
            log::info!("Field");
            f.inspect(|i| println!("{:?}", i));
            unimplemented!()
        }
        MultipartItem::Nested(mp) => {
            log::info!("Nested");
            Box::new(
                mp.map_err(error::ErrorInternalServerError)
                    .map(handle_multipart_item)
                    .flatten(),
            )
        }
    }
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
