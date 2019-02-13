use crate::schema::pictures;

use chrono::naive::NaiveDateTime;
use uuid::Uuid;

/// Picture entry model that has full representation of the picture.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Picture {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
}

/// Picture entry model to insert new row into the database.
#[derive(Debug, Insertable)]
#[table_name = "pictures"]
pub struct PictureNew<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub image: &'a str,
}

// TODO: new models as needed
