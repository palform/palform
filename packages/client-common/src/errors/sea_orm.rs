use sea_orm::DbErr;

use super::error::{APIError, APIInternalError, APIInternalErrorResult};

impl APIInternalError for DbErr {
    fn to_internal_error(&self) -> APIError {
        APIError::report_internal_error("database", self)
    }
    fn to_internal_error_with_reason(&self, reason: &str) -> APIError {
        APIError::report_internal_error(reason, self)
    }
}

impl<T> APIInternalErrorResult<T> for Result<T, DbErr> {
    fn map_internal_error(self) -> Result<T, APIError> {
        self.map_err(|e| e.to_internal_error())
    }
}
