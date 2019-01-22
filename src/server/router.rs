use crate::db;
use crate::server::handlers::*;
use router::Router;
use std::sync::{Arc, Mutex};

pub fn init(db: &Arc<Mutex<db::Database>>) -> Router {
    // Supported methods: get, post, put, delete, head, patch, options and any.
    router!(
        get_users:              get     "/pictures"         => pictures::GetHandler         { db: db.clone() }, // get picture list
        get_user_with_login:    get     "/pictures/:id:"    => pictures::GetLoginHandler    { db: db.clone() }, // get single picture
        add_user:               put     "/pictures"         => pictures::PutHandler         { db: db.clone() }, // upload picture
        delete_user:            delete  "/pictures/:id"     => pictures::DeleteLoginHandler { db: db.clone() }, // delete picture
    )
}
