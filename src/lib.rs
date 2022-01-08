use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// responder trait means that it can be converted to HttpResponse wi=hic is available
// by default for common types
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// We return `Server` on th happy path we dropped the `async` keyword
// we have no await call build server and return it, main is awaiting on it
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // web::get() is shortcut for
            // Route::new().guard(guard::get())
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    // return that the server built and bound
    Ok(server)
}
