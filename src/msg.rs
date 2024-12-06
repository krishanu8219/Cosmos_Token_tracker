use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::QueryResponses;

// Define ExecuteMsg for the execute entry point
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPreferences { tokens: Vec<String> },
}

// Define InstantiateMsg for the instantiation entry point
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct InstantiateMsg {}

// Define QueryMsg for the query entry point
#[derive(Serialize, Deserialize, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(Vec<String>)]
    GetPreferences { user: String },
}

// Define responses for query messages
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PreferencesResponse {
    pub tokens: Vec<String>,
}
