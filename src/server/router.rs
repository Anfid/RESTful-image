use crate::db;
use crate::server::handlers::*;
use router::{router, Router};

pub fn init(db: db::Database) -> Router {
    // Supported methods: get, post, put, delete, head, patch, options and any.
    router!(
        post_picture:            post    "/pictures"         => pictures::PostHandler         { db: db }, // upload picture
    )
}
