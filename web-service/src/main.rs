use std::{error::Error, fmt::Display};

use actix_web::{HttpServer, App, web, HttpRequest, Responder, HttpResponse, ResponseError, http::StatusCode};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/user", web::post().to(ingest_data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    "Hello world!"
}

#[derive(Debug)]
enum IngestError {
    BadUsername,
    Other(String),
}

impl ResponseError for IngestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            &IngestError::BadUsername => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            IngestError::BadUsername => HttpResponse::BadRequest().body(""),
            IngestError::Other(s) => HttpResponse::InternalServerError().body(s),
        }
    }
}

impl Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
struct Info {
    user: String,
}

async fn ingest_data(info: web::Json<Info>) -> Result<HttpResponse, IngestError> {
    Ok(
        HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", info.user))
    )
}


