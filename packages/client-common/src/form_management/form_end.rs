use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Serialize, Deserialize)]
pub struct APIFormEndConfiguration {
    pub message: Option<String>,
    pub redirect_to: Option<String>,
    pub show_restart: bool,
}

#[cfg(feature = "backend")]
impl sea_orm::TryGetable for APIFormEndConfiguration {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::prelude::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let json_value: serde_json::Value = res
            .try_get_by(index)
            .map_err(|e| sea_orm::TryGetError::DbErr(e))?;

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
