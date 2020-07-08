use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LambdaResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    headers: HashMap<String, String>,
}

pub struct LambdaResponseBuilder {
    status_code: u16,
    headers: HashMap<String, String>,
}

impl LambdaResponseBuilder {
    pub fn new() -> Self {
        LambdaResponseBuilder {
            status_code: 200,
            headers: HashMap::new(),
        }
    }

    pub fn with_status(mut self, code: u16) -> Self {
        self.status_code = code;
        self
    }

    pub fn with_header<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.headers
            .insert(name.into().to_ascii_lowercase(), value.into());
        self
    }

    pub fn build(self) -> LambdaResponse {
        LambdaResponse {
            is_base64_encoded: false,
            status_code: self.status_code,
            headers: self.headers,
        }
    }
}
