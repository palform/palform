use sea_orm::DbErr;
use stripe::{ParseIdError, StripeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BillingError {
    #[error("Parse ID: {0}")]
    ParseID(#[from] ParseIdError),
    #[error("Stripe: {0}")]
    Stripe(#[from] StripeError),
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Field cannot be None: {0}")]
    FieldNone(String),
    #[error("Requires setup: {0}")]
    NotSetUp(String),
    #[error("That billing frequency is not supported")]
    UnsupportedFrequency,
    #[error("Cannot perform operation: {0}")]
    BadState(String),
    #[error("Not allowed")]
    NotAllowed,
}
