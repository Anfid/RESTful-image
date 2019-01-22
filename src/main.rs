#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;

mod db;
mod logger;
mod models;
mod schema;
mod server;

use crate::db::Database;
use std::sync::{Arc, Mutex};

fn main() {
    logger::init();

    server::serve(&Arc::new(Mutex::new(Database::init())));
}
