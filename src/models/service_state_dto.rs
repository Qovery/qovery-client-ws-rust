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
pub enum ServiceStateDto {
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "WARNING")]
    Warning,

}

impl std::fmt::Display for ServiceStateDto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Starting => write!(f, "STARTING"),
            Self::Running => write!(f, "RUNNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Stopping => write!(f, "STOPPING"),
            Self::Stopped => write!(f, "STOPPED"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Warning => write!(f, "WARNING"),
        }
    }
}

impl Default for ServiceStateDto {
    fn default() -> ServiceStateDto {
        Self::Starting
    }
}

