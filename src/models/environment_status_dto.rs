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
pub struct EnvironmentStatusDto {
    #[serde(rename = "applications")]
    pub applications: Vec<models::ApplicationStatusDto>,
    #[serde(rename = "containers")]
    pub containers: Vec<models::ApplicationStatusDto>,
    #[serde(rename = "databases")]
    pub databases: Vec<models::DatabaseStatusDto>,
    #[serde(rename = "helms")]
    pub helms: Vec<models::ApplicationStatusDto>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "jobs")]
    pub jobs: Vec<models::ApplicationStatusDto>,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "state")]
    pub state: models::ServiceStateDto,
    #[serde(rename = "terraform")]
    pub terraform: Vec<models::ApplicationStatusDto>,
}

impl EnvironmentStatusDto {
    pub fn new(applications: Vec<models::ApplicationStatusDto>, containers: Vec<models::ApplicationStatusDto>, databases: Vec<models::DatabaseStatusDto>, helms: Vec<models::ApplicationStatusDto>, id: String, jobs: Vec<models::ApplicationStatusDto>, project_id: String, state: models::ServiceStateDto, terraform: Vec<models::ApplicationStatusDto>) -> EnvironmentStatusDto {
        EnvironmentStatusDto {
            applications,
            containers,
            databases,
            helms,
            id,
            jobs,
            project_id,
            state,
            terraform,
        }
    }
}

