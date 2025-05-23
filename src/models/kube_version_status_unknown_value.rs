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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeVersionStatusUnknownValue {
    #[serde(rename = "type")]
    pub r#type: String,
}

impl KubeVersionStatusUnknownValue {
    pub fn new(r#type: String) -> KubeVersionStatusUnknownValue {
        KubeVersionStatusUnknownValue {
            r#type,
        }
    }
}

