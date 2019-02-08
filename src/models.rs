use crate::schema::pictures;

use actix::prelude::*;
use actix_web::Error;
use chrono::naive::NaiveDateTime;

/// Picture entry model that has full representation of the picture.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Picture {
    pub id: i64,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
}

/// Picture entry model to insert new row into the database.
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "pictures"]
pub struct PictureNew<'a> {
    pub name: &'a str,
    pub image: &'a str,
}

// TODO: new models as needed
