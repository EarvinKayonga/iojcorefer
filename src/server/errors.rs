use actix_web::{error, http, HttpResponse};

#[derive(Fail, Debug)]
pub enum FetchError {
    #[fail(display = "internal error")]
    StoreError,
    #[fail(display = "entry not found")]
    NotFoundError,
    #[fail(display = "no id provided")]
    NoProvidedIdError,
    #[fail(display = "unparseable id")]
    IDParsingError,
}

impl error::ResponseError for FetchError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            FetchError::StoreError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            FetchError::NoProvidedIdError => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            FetchError::NotFoundError => HttpResponse::new(http::StatusCode::NOT_FOUND),
            FetchError::IDParsingError => HttpResponse::new(http::StatusCode::UNPROCESSABLE_ENTITY),
        }
    }
}
