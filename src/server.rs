mod handlers;

use crate::db::Database;
use actix_web::{server, App, HttpRequest};
use listenfd::ListenFd;

fn index(_req: &HttpRequest<Database>) -> &'static str {
    // TODO
    "Hello world"
}

pub fn serve() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(Database::init()).resource("/pictures", |r| {
            r.get().f(index);
            r.post().f(handlers::pictures::post);
        })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("localhost:7878").unwrap()
    };

    server.run();
}
