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
pub struct ServiceListPodsResponseDto {
    #[serde(rename = "pods")]
    pub pods: Vec<models::PodDto>,
}

impl ServiceListPodsResponseDto {
    pub fn new(pods: Vec<models::PodDto>) -> ServiceListPodsResponseDto {
        ServiceListPodsResponseDto {
            pods,
        }
    }
}

