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

pub struct ApiResponseBuilder {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl ApiResponseBuilder {
    pub fn new() -> Self {
        ApiResponseBuilder {
            status_code: 204,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn from_response(response: ApiResponse) -> Self {
        ApiResponseBuilder {
            status_code: response.status_code,
            headers: response.headers.unwrap_or_default(),
            body: response.body,
        }
    }

    pub fn with_status(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_header(mut self, header: &str, header_value: &str) -> Self {
        self.headers
            .insert(header.to_string(), header_value.to_string());
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    pub fn build(&self) -> ApiResponse {
        let headers = if self.headers.is_empty() {
            None
        } else {
            Some(self.headers.clone())
        };

        ApiResponse {
            is_base64_encoded: false,
            status_code: self.status_code,
            headers,
            body: self.body.clone(),
        }
    }
}
