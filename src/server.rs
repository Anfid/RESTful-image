mod handlers;

use crate::db::DbExecutor;
use actix::prelude::*;
use actix_web::{http, pred, server, App, HttpRequest, HttpResponse};
use listenfd::ListenFd;

pub struct AppState {
    db: Addr<DbExecutor>,
}

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    // TODO: describe API
    "Hello world"
}

pub fn serve() {
    let sys = actix::System::new("server");

    let mut listenfd = ListenFd::from_env();

    let addr = DbExecutor::init();

    let mut server = server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .resource("/pictures", |resource| {
                resource
                    .method(http::Method::POST)
                    .filter(pred::Header("content-type", "application/json"))
                    .f(|_req| HttpResponse::Ok());

                resource
                    .method(http::Method::POST)
                    .f(handlers::pictures::handle_multipart);
            })
            .resource("/", |resource| resource.method(http::Method::GET).f(index))
            .finish()
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("localhost:7878").unwrap()
    };
    server.start();

    sys.run();
}
