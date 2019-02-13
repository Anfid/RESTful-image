//! Database interface

use crate::models::*;
use crate::schema::*;
use actix::prelude::*;
use actix_web::{error, Error};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone)]
pub struct DbExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct PictureCreate {
    pub name: String,
    pub image: String,
}

impl Message for PictureCreate {
    type Result = Result<Picture, Error>;
}

/// Inserts picture into the database.
impl Handler<PictureCreate> for DbExecutor {
    type Result = Result<Picture, Error>;

    fn handle(&mut self, msg: PictureCreate, _: &mut Self::Context) -> Self::Result {
        let uuid = uuid::Uuid::new_v4();

        let pic_model = PictureNew {
            id: &uuid,
            name: &msg.name,
            image: &msg.image,
        };

        diesel::insert_into(pictures::table)
            .values(&pic_model)
            .get_result(&self.pool.get().unwrap())
            .map_err(|e| {
                log::error!("Error saving image: {}", e);
                error::ErrorInternalServerError("Could not save the image")
            })
    }
}

impl DbExecutor {
    /// Initializes database (TODO with supplied parameters).
    pub fn init() -> Addr<DbExecutor> {
        let manager: ConnectionManager<PgConnection> =
            ConnectionManager::new("postgres://postgres:postgres@localhost");

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        let addr = SyncArbiter::start(4, move || DbExecutor { pool: pool.clone() });

        log::debug!("Database connection established");
        addr
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

    /// Removes picture from database
    pub fn delete(&self, name: &str) -> QueryResult<Picture> {
        diesel::delete(pictures::table.filter(pictures::name.eq(name)))
            .get_result(&self.pool.get().unwrap())
    }
}
