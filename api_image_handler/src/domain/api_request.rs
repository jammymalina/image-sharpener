use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct PathParameters {
    pub operation: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiRequest {
    pub headers: HashMap<String, String>,
    pub path_parameters: PathParameters,
    pub body: Option<String>,
}
