use crate::db::Database;
use crate::models::PictureNew;
use bodyparser;
use iron::prelude::*;
use iron::{status, Handler, IronResult, Request, Response};
use serde_json;
use std::sync::{Arc, Mutex};
use url::Url;

/// Handler for GET /pictures
/// Lists all stored pictures
// TODO
pub struct GetHandler {
    pub db: Arc<Mutex<Database>>,
}
impl Handler for GetHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let url: Url = req.url.clone().into();
        let pairs = url.query_pairs();
        if pairs.count() != 0 {
            let mut login = None;
            for (key, value) in pairs {
                match key.into_owned().as_str() {
                    "login" => login = Some(value.into_owned()),
                    _ => return Ok(Response::with((status::BadRequest, "passed names don't parse as application/x-www-form-urlencoded or there are no parameters"))),
                }
            }
            if let Some(login) = login {
                if let Some(user) = self.db.lock().unwrap().get_user(login.as_str()) {
                    return Ok(Response::with((
                        status::Ok,
                        serde_json::to_string(&UserPubFull::from(&user)).unwrap(),
                    )));
                } else {
                    return Ok(Response::with((status::NotFound, "User not found")));
                }
            } else {
                // TODO: search by different fields
                return Ok(Response::with((
                    status::NotImplemented,
                    "Search by other than login field is not yet implemented",
                )));
            }
        } else {
            let users: Vec<UserPubBrief> = self
                .db
                .lock()
                .unwrap()
                .select(None)
                .into_iter()
                .map(|u| UserPubBrief::from(&u))
                .collect();
            Ok(Response::with((
                status::Ok,
                serde_json::to_string(&users).unwrap(),
            )))
        }
    }
}

/// Handler for GET /users/:login
/// Shows user with login
pub struct GetLoginHandler {
    pub db: Arc<Mutex<Database>>,
}
impl Handler for GetLoginHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        match self.db.lock().unwrap().get_user(path[1]) {
            Some(user) => Ok(Response::with((
                status::Ok,
                serde_json::to_string(&UserPubFull::from(&user)).unwrap(),
            ))),
            None => Ok(Response::with((status::NotFound, "User not found"))),
        }
    }
}

/// Handler for PUT /pictures
pub struct PutHandler {
    pub db: Arc<Mutex<Database>>,
}
impl Handler for PutHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let url: Url = req.url.clone().into();
        let pairs = url.query_pairs();
        let mut login = None;
        let mut password = None;
        let mut description = None;
        for (key, value) in pairs {
            match key.into_owned().as_str() {
                "login" => login = Some(value.into_owned()),
                "password" => password = Some(value.into_owned()),
                "description" => description = Some(value.into_owned()),
                _ => error!("Wrong field"),
            }
        }
        if login == None || password == None {
            Ok(Response::with((
                status::BadRequest,
                "Required fields not provided",
            )))
        } else {
            let user = UserNew {
                login: login.unwrap(),
                password: password.unwrap(),
                description,
            };
            match self.db.lock().unwrap().insert(user) {
                Ok(user) => Ok(Response::with((
                    status::Created,
                    serde_json::to_string(&UserPubFull::from(&user)).unwrap(),
                ))),
                // TODO: Specify HTML status code
                Err(e) => Ok(Response::with((
                    status::InternalServerError,
                    format!("{}", e),
                ))),
            }
        }
    }
}

/// Handler for DELETE /users/:login
/// Deletes user with login
pub struct DeleteLoginHandler {
    pub db: Arc<Mutex<Database>>,
}
impl Handler for DeleteLoginHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let user = parse_body(req);
        match self.db.lock().unwrap().delete(user.login.as_str()) {
            Ok(u) => Ok(Response::with((
                status::Ok,
                serde_json::to_string(&UserPubFull::from(&u)).unwrap(),
            ))),
            Err(e) => Ok(Response::with((
                status::NotFound,
                format!("Error deleting user: {}", e),
            ))),
        }
    }
}

/// Returns parsed into a serializable structure body
/// Currently returns models::UserNew struct
/// TODO: Use special struct for requests' and responses' bodies
fn parse_body(req: &mut Request) -> UserNew {
    // Should not fail. TODO: Test more
    let body = req
        .get::<bodyparser::Raw>()
        .expect("Unexpected fail in parse_body");
    match body.as_ref() {
        Some(body) => trace!("Read body:\n{}", body),
        None => error!("No body"),
    }

    serde_json::from_str(body.unwrap().as_str()).unwrap()
}
