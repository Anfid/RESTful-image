use crate::db::{DbExecutor, PictureCreate};
use crate::models::Picture;
use crate::server::AppState;
use actix::Addr;
use actix_web::multipart::*;
use actix_web::{dev, error, http, FutureResponse, HttpMessage, HttpRequest, HttpResponse};
use futures::future::Future;
use futures::*;
use serde_json;

fn content_type_is_multipart(req: &HttpRequest<AppState>) -> bool {
    match req.headers().get(http::header::CONTENT_TYPE) {
        Some(t) => match t.to_str() {
            Ok(t) => t.starts_with("multipart/form-data"),
            Err(e) => {
                log::error!("Error reading header: {}", e);
                false
            }
        },
        None => false,
    }
}

/// Handler for POST /pictures
pub fn handle_multipart(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    if content_type_is_multipart(req) {
        log::info!("multipart/form-data");
        let db = req.state().db.clone();
        Box::new(
            req.multipart()
                .map_err(actix_web::error::ErrorInternalServerError)
                .map(move |item| handle_multipart_item(db.clone(), item))
                .flatten()
                .collect()
                .map(|pictures| HttpResponse::Created().json(pictures))
                .map_err(|e| {
                    log::error!("Failed: {}", e);
                    e
                }),
        )
    } else {
        Box::new(futures::future::ok(HttpResponse::BadRequest().body(
            "Content-Type must be either application/json or multipart/form-data",
        )))
    }

    /*
    HttpResponse::Created().json(save(
        req.state(),
        PictureNew {
            name: "1",
            image: "2",
            description: None,
        },
    ))
    */
}

fn handle_multipart_item(
    db: Addr<DbExecutor>,
    item: MultipartItem<dev::Payload>,
) -> Box<Stream<Item = Picture, Error = actix_web::Error>> {
    log::info!("Handling multipart item");
    match item {
        MultipartItem::Field(field) => {
            log::info!("Field");

            let result = field
                .map_err(actix_web::error::ErrorInternalServerError)
                .fold(Vec::new(), |acc, bytes| {
                    let mut acc = acc;
                    acc.extend(&bytes);
                    future::ok::<_, actix_web::Error>(acc)
                })
                .and_then(move |bytes_vec| {
                    db.send(PictureCreate {
                        name: "1".to_owned(),
                        image: base64::encode(&bytes_vec),
                    })
                    .from_err()
                    .and_then(|send_result| match send_result {
                        Ok(picture) => future::ok(picture),
                        Err(err) => future::err(err),
                    })
                });

            Box::new(result.into_stream())
        }
        MultipartItem::Nested(mp) => {
            log::info!("Nested");
            Box::new(
                mp.map_err(error::ErrorInternalServerError)
                    .map(move |item| handle_multipart_item(db.clone(), item))
                    .flatten(),
            )
        }
    }
}

pub fn handle_json() -> HttpResponse {
    unimplemented!()
}
