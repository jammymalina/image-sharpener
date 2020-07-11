use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub is_base64_encoded: bool,
    pub status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
