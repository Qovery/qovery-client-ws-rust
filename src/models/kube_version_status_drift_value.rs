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
pub struct KubeVersionStatusDriftValue {
    #[serde(rename = "expected_kube_version")]
    pub expected_kube_version: String,
    #[serde(rename = "kube_version")]
    pub kube_version: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl KubeVersionStatusDriftValue {
    pub fn new(expected_kube_version: String, kube_version: String, r#type: String) -> KubeVersionStatusDriftValue {
        KubeVersionStatusDriftValue {
            expected_kube_version,
            kube_version,
            r#type,
        }
    }
}

