mod handlers;

use crate::db::Database;
use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest<Database>) -> &'static str {
    // TODO
    "Hello world"
}

pub fn serve() {
    server::new(|| {
        App::with_state(Database::init()).resource("/pictures", |r| {
            r.get().f(index);
            r.post().f(handlers::pictures::post);
        })
    })
    .bind("localhost:7878")
    .unwrap()
    .run();
}
