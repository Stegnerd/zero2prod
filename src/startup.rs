use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

// We return `Server` on th happy path we dropped the `async` keyword
// we have no await call build server and return it, main is awaiting on it
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // web::get() is shortcut for
            // Route::new().guard(guard::get())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // rather than bind we listen for which addr comes in
    .listen(listener)?
    .run();

    // return that the server built and bound
    Ok(server)
}
