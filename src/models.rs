use crate::schema::pictures;

use chrono::naive::NaiveDateTime;
use uuid::Uuid;

/// Picture model with all picture data.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Picture {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
}

/// Picture model without image data.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct PictureBrief {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

/// Insertable picture model.
#[derive(Debug, Insertable)]
#[table_name = "pictures"]
pub struct PictureNew<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub image: &'a str,
}
