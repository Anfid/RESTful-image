#![allow(dead_code)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod db;
mod logger;
mod models;
mod schema;
mod server;

fn main() {
    logger::init();

    server::serve();
}
