/*
 * websocket-gateway
 *
 * Describe the websocket endpoints of Qovery
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: erebe@erebe.eu
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResourceStatusDto {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ALERT")]
    Alert,

}

impl std::fmt::Display for ResourceStatusDto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Warning => write!(f, "WARNING"),
            Self::Alert => write!(f, "ALERT"),
        }
    }
}

impl Default for ResourceStatusDto {
    fn default() -> ResourceStatusDto {
        Self::Ok
    }
}

