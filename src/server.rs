mod handlers;
mod router;

use crate::db;
use iron::Iron;

pub fn serve(db: db::Database) {
    let router = router::init(db);
    Iron::new(router).http("localhost:7878").unwrap();
}
