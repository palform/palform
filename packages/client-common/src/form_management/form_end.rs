use serde::{Deserialize, Serialize};

/// A configuration for what the page shown to the user after successfully submitting the form
/// should look like
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Serialize, Deserialize)]
pub struct APIFormEndConfiguration {
    /// A Markdown-enabled message to show to the user
    pub message: Option<String>,
    /// If specified, a button is shown with the text "Continue" that links to this URL
    pub redirect_to: Option<String>,
    /// Whether to show a button to start a new submission from scratch
    pub show_restart: bool,
}

#[cfg(feature = "backend")]
impl sea_orm::TryGetable for APIFormEndConfiguration {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::prelude::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let json_value: serde_json::Value =
            res.try_get_by(index).map_err(sea_orm::TryGetError::DbErr)?;

        let parsed = serde_json::from_value(json_value)
            .map_err(|e| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Json(e.to_string())))?;

        Ok(parsed)
    }
}

impl Default for APIFormEndConfiguration {
    fn default() -> Self {
        Self {
            message: Some(
                "Thanks for completing this form! Your response has been sent securely."
                    .to_string(),
            ),
            redirect_to: None,
            show_restart: true,
        }
    }
}
