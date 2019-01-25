use crate::schema::pictures;
use chrono::naive::NaiveDateTime;

/// Picture entry model to insert new row into the database.
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "pictures"]
pub struct PictureNew {
    pub name: String,
    pub image: String,
    pub description: Option<String>,
}

/// Picture entry model that has full representation of the picture.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Picture {
    pub id: i64,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
    pub description: Option<String>,
}

// TODO: new models as needed
