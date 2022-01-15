use actix_web::HttpResponse;

// responder trait means that it can be converted to HttpResponse wi=hic is available
// by default for common types
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
