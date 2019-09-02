mod handlers;

use crate::db::DbExecutor;
use actix::prelude::*;
use actix_web::{http, web, App, HttpServer};
use listenfd::ListenFd;

pub struct AppState {
    db_actor: Addr<DbExecutor>,
}

fn index(_: web::Payload) -> &'static str {
    r#"Brief API description

GET / - see this page
POST /pictures <multipart>
POST /pictures <json>

Allowed json format:
["name": "<name>", "base64": "<base64>", ...]
["name": "<name>", "url": "<url>", ...]
"#
}

pub fn serve() {
    let sys = actix::System::new("server");

    let mut listenfd = ListenFd::from_env();

    let addr = DbExecutor::init();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/pictures")
                    .route(web::post().to_async(handlers::pictures::handle_multipart)),
            )
    });

    /*
    let mut server = server::new(move || {
        App::with_state(AppState {
            db_actor: addr.clone(),
        })
        .resource("/pictures/{uuid}", |resource| {
            resource.method(http::Method::GET).f(index);
        })
        .resource("/pictures", |resource| {
            resource
                .method(http::Method::POST)
                .filter(pred::Header("content-type", "application/json"))
                .f(handlers::pictures::handle_json);

            resource
                .method(http::Method::POST)
                .f(handlers::pictures::handle_multipart);
        })
        .resource("/", |resource| resource.method(http::Method::GET).f(index))
        .finish()
    });
    */

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).expect("Error on server listen")
    } else {
        server.bind("localhost:7878").expect("Error on server bind")
    };
    server.start();

    sys.run().expect("Error on system run");
}
