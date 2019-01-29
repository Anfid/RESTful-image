use crate::db::Database;
use crate::models::PictureNew;
use bodyparser;
use iron::{status, Handler, IronResult, Request, Response};
use multipart::server::{Entries, Multipart, MultipartField, ReadEntry, SaveResult};
use serde_json;

/// Handler for POST /pictures
pub struct PostHandler {
    pub db: Database,
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        if let Some(t) = req.headers.get::<iron::headers::ContentType>() {
            if t == &iron::headers::ContentType::json() {
                log::debug!("Received request Content-Type: application/json");

                PostHandler::handle_json(req);
            } else if let Ok(mut multipart) = Multipart::from_request(req) {
                log::debug!("Received request Content-Type: multipart/form-data");

                match PostHandler::handle_multipart(&mut multipart) {
                    Ok(result) => return Ok(Response::with((status::Created, result))),
                    Err(e) => return Ok(Response::with((status::InternalServerError, e))),
                }
            } else {
                log::info!("Received request Content-Type unsupported");

                return Ok(Response::with((
                    status::BadRequest,
                    "Unsupported Content-Type",
                )));
            }
        } else {
            return Ok(Response::with((
                status::BadRequest,
                "Requests with unspecified Content-Type are not supported",
            )));
        }

        let picture = PictureNew {
            name: "1".to_owned(),
            image: "2".to_owned(),
            description: None,
        };

        match self.db.insert(picture) {
            Ok(picture) => Ok(Response::with((
                status::Created,
                serde_json::to_string(&picture).unwrap(),
            ))),
            Err(e) => Ok(Response::with((
                status::InternalServerError,
                format!("{}", e),
            ))),
        }
    }
}

impl PostHandler {
    fn handle_json(_req: &mut Request) {
        unimplemented!()
    }

    fn handle_multipart(multipart: &mut Multipart<impl std::io::Read>) -> Result<String, String> {
        use multipart::server::save::SavedData::*;
        if let SaveResult::Full(entries) = multipart.save().temp() {
            for (k, v) in entries.fields.iter() {
                print!("{}: ", k);
                for val in v {
                    match &val.data {
                        Text(t) => println!("{}", t),
                        Bytes(b) => println!("{:?}", b),
                        File(p, s) => println!("{:?} {} KB", p, s / 1024),
                    }
                }
            }
            Ok("".to_owned())
        } else {
            // TODO: Consider return code
            Err("/*TODO*/".to_owned())
        }
    }

    fn save(_pic: &PictureNew) -> Result<(), ()> {
        unimplemented!()
    }
}
