use super::{api_error::ApiError, error_type::ErrorType};

pub struct BadRequestError {
    error_message: String,
    error_type: ErrorType,
}

impl ApiError for BadRequestError {
    fn get_status_code(&self) -> u16 {
        400
    }

    fn get_error_type(&self) -> String {
        self.error_type.to_str().to_string()
    }

    fn get_error_message(&self) -> String {
        self.error_message.clone()
    }
}

impl BadRequestError {
    pub fn new(error_message: &str, error_type: ErrorType) -> BadRequestError {
        BadRequestError {
            error_message: error_message.to_string(),
            error_type,
        }
    }
}
