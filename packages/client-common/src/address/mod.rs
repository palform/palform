use std::fmt::Display;

use geo::Point;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "frontend-js", derive(ts_rs::TS))]
#[cfg_attr(feature = "frontend-js", ts(export))]
#[derive(Deserialize, Serialize, Clone)]
pub struct APIGenericAddress {
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    locality: Option<String>,
    iso_3166_alpha_1_code: Option<String>,
}

impl APIGenericAddress {
    pub fn is_empty(&self) -> bool {
        self.line1.is_none()
            && self.line2.is_none()
            && self.postal_code.is_none()
            && self.locality.is_none()
            && self.iso_3166_alpha_1_code.is_none()
    }

    pub fn is_in_country(&self, country_code: String) -> bool {
        self.iso_3166_alpha_1_code
            .clone()
            .is_some_and(|c| c == country_code)
    }
}

impl Display for APIGenericAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line1) = self.line1.clone() {
            writeln!(f, "{}", line1)?;
        }
        if let Some(line2) = self.line2.clone() {
            writeln!(f, "{}", line2)?;
        }
        if let Some(postal_code) = self.postal_code.clone() {
            writeln!(f, "{}", postal_code)?;
        }
        if let Some(locality) = self.locality.clone() {
            writeln!(f, "{}", locality)?;
        }
        if let Some(iso_3166_alpha_1_code) = self.iso_3166_alpha_1_code.clone() {
            writeln!(f, "{}", iso_3166_alpha_1_code)?;
        }
        Ok(())
    }
}

impl Default for APIGenericAddress {
    fn default() -> Self {
        Self {
            line1: None,
            line2: None,
            postal_code: None,
            locality: None,
            iso_3166_alpha_1_code: None,
        }
    }
}

#[cfg_attr(feature = "frontend-js", derive(ts_rs::TS))]
#[cfg_attr(feature = "frontend-js", ts(export))]
#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct APIGenericLocation {
    lat: f64,
    lng: f64,
}

impl APIGenericLocation {
    pub fn is_empty(&self) -> bool {
        self.lat == 0.0 && self.lng == 0.0
    }

    pub fn get_lat(&self) -> f64 {
        self.lat
    }

    pub fn get_lng(&self) -> f64 {
        self.lng
    }
}

impl Default for APIGenericLocation {
    fn default() -> Self {
        APIGenericLocation { lat: 0.0, lng: 0.0 }
    }
}

impl From<APIGenericLocation> for Point<f64> {
    fn from(value: APIGenericLocation) -> Self {
        Point::new(value.lat, value.lng)
    }
}
