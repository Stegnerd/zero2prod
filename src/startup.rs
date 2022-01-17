use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

// We return `Server` on th happy path we dropped the `async` keyword
// we have no await call build server and return it, main is awaiting on it
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    // arc is always clone-able, regardless of type T
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            // web::get() is shortcut for
            // Route::new().guard(guard::get())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // register connection as part of the application state
            .app_data(db_pool.clone())
    })
    // rather than bind we listen for which addr comes in
    .listen(listener)?
    .run();

    // return that the server built and bound
    Ok(server)
}
