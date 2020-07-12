use serde::Serialize;

use super::super::api_response::{ApiResponse, ApiResponseBuilder};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    error_type: String,
    error_message: String,
}

pub trait ApiError {
    fn get_status_code(&self) -> u16;
    fn get_error_type(&self) -> String;
    fn get_error_message(&self) -> String;

    fn get_http_response(&self) -> ApiResponse {
        let response_builder = ApiResponseBuilder::new();
        let error = ErrorResponse {
            error_type: self.get_error_type(),
            error_message: self.get_error_message(),
        };

        response_builder
            .with_status(self.get_status_code())
            .with_header("Content-Type", "application/json")
            .with_body(&serde_json::to_string(&error).unwrap())
            .build()
    }
}
