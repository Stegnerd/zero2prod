use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// responder trait means that it can be converted to HttpResponse wi=hic is available
// by default for common types
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// this macro generates boiler plate to wrap async around our synchronous main
// since rust main can't be synchronous
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // web::get() is shortcut for
            // Route::new().guard(guard::get())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
