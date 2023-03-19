use std::io::Error;
use std::net::TcpListener;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();
    Ok(server)
}


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}