//! Database interface

use crate::models::*;
use crate::schema::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::error::Error;

pub enum Query<'a> {
    Select(Option<String>),
    Insert(PictureNew<'a>),
    Delete(String),
}

#[derive(Clone)]
pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    /// Initializes database (TODO with supplied parameters).
    pub fn init() -> Database {
        let manager: ConnectionManager<PgConnection> =
            diesel::r2d2::ConnectionManager::new("postgres://postgres:postgres@localhost");

        let pool = diesel::r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .unwrap();

        log::debug!("Database connection established");

        Database { pool }
    }

    /// Used only for cli or debugging purposes. Prints query result to stdout
    pub fn query(&self, query: Query) {
        match query {
            Query::Select(name) => {
                let result = self.select(name);
                println!("{:?}", result);
            }
            Query::Insert(picture) => match self.insert(picture) {
                Ok(picture) => println!("Created picture: {:?}", picture),
                Err(e) => println!("Could not create picture: {}", e.description()),
            },
            Query::Delete(name) => match self.delete(name.as_str()) {
                Ok(picture) => println!("Deleted picture '{:?}'", picture),
                Err(e) => println!("Could not delete picture: '{}'", e.description()),
            },
        }
    }

    pub fn get_picture(&self, name: &str) -> Option<Picture> {
        let mut result = pictures::table
            .filter(pictures::name.eq(name))
            .limit(1)
            .load::<Picture>(&self.pool.get().unwrap())
            .expect("Could not get picture");

        result.pop()
    }

    /// Lists all database pictures if `Option::None` is provided. Else print picture with specified
    /// name.
    pub fn select(&self, name: Option<String>) -> Vec<Picture> {
        let mut matches = pictures::table.into_boxed();
        if let Some(name) = name {
            matches = matches.filter(pictures::name.eq(name));
        }

        matches
            .load::<Picture>(&self.pool.get().unwrap())
            .expect("Could not get pictures")
    }

    /// Inserts picture into the database.
    pub fn insert(&self, picture: PictureNew) -> QueryResult<Picture> {
        diesel::insert_into(pictures::table)
            .values(&picture)
            .get_result(&self.pool.get().unwrap())
    }

    /// Removes picture from database
    pub fn delete(&self, name: &str) -> QueryResult<Picture> {
        diesel::delete(pictures::table.filter(pictures::name.eq(name)))
            .get_result(&self.pool.get().unwrap())
    }
}
