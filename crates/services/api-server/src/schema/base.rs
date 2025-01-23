use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Result {
    pub data: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ErrorResult {
    pub err_msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_detail: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pagination {
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

pub(crate) trait PostValidate {
    async fn is_valid(&self) -> bool;
}
