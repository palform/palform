use actix_web::{http::header::ContentType, HttpResponse, Responder};

pub async fn not_found_handler() -> impl Responder {
    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("not found")
}
