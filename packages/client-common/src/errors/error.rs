use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
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
pub type APIErrorWithStatus = (rocket::http::Status, rocket::serde::json::Json<APIError>);

#[cfg(feature = "backend")]
impl APIError {
    pub fn get_status(&self) -> rocket::http::Status {
        match self {
            Self::BadRequest(_) => rocket::http::Status::BadRequest,
            Self::ValidationError(_) => rocket::http::Status::UnprocessableEntity,
            Self::CaptchaError(_) => rocket::http::Status::BadRequest,
            Self::NotAllowed => rocket::http::Status::Forbidden,
            Self::SubscriptionLimit(_) => rocket::http::Status::PaymentRequired,
            Self::Internal => rocket::http::Status::InternalServerError,
            Self::NotFound => rocket::http::Status::NotFound,
        }
    }

    fn response(&self) -> APIErrorWithStatus {
        (self.get_status(), rocket::serde::json::Json(self.clone()))
    }

    pub fn report_internal_error<E: std::error::Error>(
        context: &str,
        error: E,
    ) -> APIErrorWithStatus {
        log::error!("{}: {}", context, error);
        APIError::Internal.into()
    }

    pub fn report_internal_error_without_error(message: &str) -> APIErrorWithStatus {
        log::error!("{}", message);
        APIError::Internal.into()
    }
}

#[cfg(feature = "backend")]
impl From<APIError> for APIErrorWithStatus {
    fn from(value: APIError) -> Self {
        value.response()
    }
}

#[cfg(feature = "backend")]
pub trait APIInternalError: std::error::Error {
    fn to_internal_error(&self) -> APIErrorWithStatus;
    fn to_internal_error_with_reason(&self, _reason: &str) -> APIErrorWithStatus {
        Self::to_internal_error(self)
    }
}

#[cfg(feature = "backend")]
pub trait APIInternalErrorResult<T> {
    fn map_internal_error(self) -> Result<T, APIErrorWithStatus>;
}
