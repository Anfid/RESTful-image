use crate::db::Database;
use crate::models::PictureNew;
use bodyparser;
use iron::{status, Handler, IronResult, Request, Response};
use serde_json;

/// Handler for POST /pictures
pub struct PostHandler {
    pub db: Database,
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        //let url: Url = req.url.clone().into();
        let has_type = req.headers.has::<iron::headers::ContentType>();
        log::info!("Has type: {}", has_type);

        if !has_type {
            return Ok(Response::with((status::BadRequest, "/*TODO*/")));
        }

        // TODO: cleanup this mess
        let req_type = req
            .headers
            .get::<iron::headers::ContentType>()
            .expect("Unspecified Content-Type");

        use iron::mime::*;
        if req_type == &iron::headers::ContentType::json() {
            log::info!("Header type: application/json");
            post_from_base64(req)
        } else if req_type
            == &iron::headers::ContentType(Mime(TopLevel::Multipart, SubLevel::FormData, vec![]))
        {
            log::info!("Header type: multipart/form-data");
            post_from_raw(req)
        } else {
            log::error!("Header type unknown");
            return Ok(Response::with((status::BadRequest, "/*TODO*/")));
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

fn post_from_base64(_req: &mut Request) {
    unimplemented!()
}

fn post_from_raw(req: &mut Request) {
    match multipart::server::Multipart::from_request(req)
        .expect("from raw 1")
        .into_entry()
    {
        multipart::server::ReadEntryResult::Entry(e) => log::info!("Entry"),
        multipart::server::ReadEntryResult::End(m) => log::info!("End"),
        multipart::server::ReadEntryResult::Error(m, e) => log::info!("Error"),
    }
}
