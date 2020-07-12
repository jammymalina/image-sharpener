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

impl ApiRequest {
    pub fn extract_headers(&self) -> HashMap<String, String> {
        let headers = self.headers.clone();
        headers
            .into_iter()
            .map(|(header, header_value)| (header.to_ascii_lowercase(), header_value))
            .collect()
    }
}
