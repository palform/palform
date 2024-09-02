use sea_orm::DbErr;

use super::error::{APIError, APIErrorWithStatus, APIInternalError, APIInternalErrorResult};

impl APIInternalError for DbErr {
    fn to_internal_error(&self) -> APIErrorWithStatus {
        APIError::report_internal_error("database", self)
    }
    fn to_internal_error_with_reason(&self, reason: &str) -> APIErrorWithStatus {
        APIError::report_internal_error(reason, self)
    }
}

impl<T> APIInternalErrorResult<T> for Result<T, DbErr> {
    fn map_internal_error(self) -> Result<T, APIErrorWithStatus> {
        self.map_err(|e| e.to_internal_error())
    }
}
