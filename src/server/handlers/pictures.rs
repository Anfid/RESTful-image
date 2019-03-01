use crate::db::{DbExecutor, PictureCreate};
use crate::models::PictureBrief;
use crate::server::AppState;
use actix::Addr;
use actix_web::multipart::*;
use actix_web::{dev, error, http, FutureResponse, HttpMessage, HttpRequest, HttpResponse};
use futures::future::Future;
use futures::*;
use serde_json;

const MAX_SIZE: usize = 67; //_108_864; // max payload size is 64MB

// Required because currently actix has limited support for multipart request filtering.
// See https://github.com/actix/actix-web/issues/693
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

#[derive(Serialize)]
enum UploadResponse {
    Success(PictureBrief),
    Fail(UploadError),
}

#[derive(Serialize)]
struct UploadError {
    message: String,
}

/// Handler for POST /pictures
pub fn handle_multipart(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    if content_type_is_multipart(req) {
        log::info!("multipart/form-data");
        let db = req.state().db_actor.clone();
        Box::new(
            req.multipart()
                .map_err(|e| e.to_string())
                .map(move |item| handle_multipart_item(db.clone(), item))
                .flatten()
                .map(UploadResponse::Success)
                .or_else(|e| {
                    log::warn!("Unable to handle request: {}", e);
                    future::ok(UploadResponse::Fail(UploadError { message: e }))
                })
                .collect()
                .map(|result| HttpResponse::Ok().json(result)),
        )
    } else {
        Box::new(futures::future::ok(HttpResponse::BadRequest().body(
            "Request Content-Type must be either application/json or multipart/form-data",
        )))
    }
}

fn handle_multipart_item(
    db: Addr<DbExecutor>,
    item: MultipartItem<dev::Payload>,
) -> Box<Stream<Item = PictureBrief, Error = String>> {
    log::info!("Handling multipart item");
    match item {
        MultipartItem::Field(field) => {
            let filename = if let Some(headers) = field.content_disposition() {
                headers
                    .get_filename()
                    .or_else(|| headers.get_name())
                    .map(|s| s.to_owned())
            } else {
                None
            };

            if field.content_type().type_() != mime::IMAGE {
                return Box::new(
                    future::err::<_, String>(String::from(
                        "Content-Type of multipart field is expected to be image",
                    ))
                    .into_stream(),
                );
            }
            log::info!("{}", field.content_type().subtype());
            let ext = field.content_type().subtype().as_str().to_owned();

            let result = field
                .map_err(|e| e.to_string())
                .fold(Vec::new(), |acc, bytes| {
                    let mut acc = acc;
                    acc.extend(&bytes);
                    future::ok::<_, String>(acc)
                })
                .and_then(move |bytes_vec| {
                    db.send(PictureCreate {
                        name: filename,
                        ext,
                        image: base64::encode(&bytes_vec),
                    })
                    .map_err(|_| String::from("Internal server error"))
                    .and_then(|send_result| match send_result {
                        Ok(picture) => future::ok(picture),
                        Err(err) => future::err(err),
                    })
                });

            Box::new(result.into_stream())
        }
        MultipartItem::Nested(mp) => Box::new(
            mp.map_err(|e| e.to_string())
                .map(move |item| handle_multipart_item(db.clone(), item))
                .flatten(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Parced {
    Url { name: String, url: String },
    Base64 { name: String, base64: String },
}

pub fn handle_json(
    req: &HttpRequest<AppState>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    log::info!("application/json");
    let db = req.state().db_actor.clone();
    Box::new(
        req.json()
            .limit(MAX_SIZE)
            .map_err(|_| error::ErrorBadRequest("Payload size should be less than 64MB"))
            .and_then(|json: Vec<Parced>| {
                log::info!("model: {:?}", json);

                json.iter()
                    .map(move |item| handle_json_item(db.clone(), item))
                    .map(|a| a.map(UploadResponse::Success)); // TODO: Any better solutions?

                Ok(HttpResponse::Ok().json(json))
            }),
    )
}

fn handle_json_item(
    db: Addr<DbExecutor>,
    item: &Parced,
) -> Box<Future<Item = PictureBrief, Error = String>> {
    let res = match item {
        Parced::Url { name, url } => unimplemented!(),
        Parced::Base64 { name, base64 } => db
            .send(PictureCreate {
                name: Some(name.to_owned()),
                ext: "jpg".to_owned(),
                image: base64.to_owned(),
            })
            .map_err(|_| String::from("Internal server error"))
            .and_then(|send_result| match send_result {
                Ok(picture) => future::ok(picture),
                Err(err) => future::err(err),
            }),
    };
    Box::new(res)
}
