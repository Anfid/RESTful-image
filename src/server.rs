mod handlers;
mod router;

use crate::db;
use iron::Iron;
use std::sync::{Arc, Mutex};

pub fn serve(db: &Arc<Mutex<db::Database>>) {
    let router = router::init(db);
    Iron::new(router).http("localhost:7878").unwrap();
}
