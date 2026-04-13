#[cfg(feature = "backend")]
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "backend",
    derive(schemars::JsonSchema, apistos::ApiErrorComponent)
)]
#[cfg_attr(
    feature = "backend",
    openapi_error(
        status(code = 400),
        status(code = 422),
        status(code = 400),
        status(code = 403),
        status(code = 402),
        status(code = 500),
        status(code = 404)
    )
)]
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum APIError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Failed validation: {0}")]
    ValidationError(String),
    #[error("Captcha: {0}")]
    CaptchaError(String),
    #[error("Not allowed")]
    NotAllowed,
    #[error("Subscription limit exceeded: {0}")]
    SubscriptionLimit(String),
    #[error("Internal")]
    Internal,
    #[error("Not found")]
    NotFound,
}

#[cfg(feature = "backend")]
impl APIError {
    pub fn report_internal_error<E: std::error::Error>(context: &str, error: E) -> APIError {
        log::error!("{}: {}", context, error);
        APIError::Internal.into()
    }

    pub fn report_internal_error_without_error(message: &str) -> APIError {
        log::error!("{}", message);
        APIError::Internal.into()
    }
}

#[cfg(feature = "backend")]
impl ResponseError for APIError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::CaptchaError(_) => StatusCode::BAD_REQUEST,
            Self::NotAllowed => StatusCode::FORBIDDEN,
            Self::SubscriptionLimit(_) => StatusCode::PAYMENT_REQUIRED,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[cfg(feature = "backend")]
pub trait APIInternalError: std::error::Error {
    fn to_internal_error(&self) -> APIError;
    fn to_internal_error_with_reason(&self, _reason: &str) -> APIError {
        Self::to_internal_error(self)
    }
}

#[cfg(feature = "backend")]
pub trait APIInternalErrorResult<T> {
    fn map_internal_error(self) -> Result<T, APIError>;
}
