use super::api_request::ApiRequest;
use super::errors::{
    api_error::ApiError, bad_request_error::BadRequestError, error_type::ErrorType,
};

pub struct ApiValidator {
    event: ApiRequest,
}

impl ApiValidator {
    pub fn new(event: &ApiRequest) -> Self {
        ApiValidator {
            event: event.clone(),
        }
    }

    fn validate_header(
        &self,
        header_name: &str,
        expected_header_value: &[&str],
    ) -> Result<(), Box<dyn ApiError>> {
        let headers = self.event.extract_headers();
        let header = headers.get(header_name);
        match header {
            Some(h) if !expected_header_value.to_vec().contains(&h.as_str()) => {
                Err(Box::new(BadRequestError::new(
                    &format!(
                        "Invalid {} header, the supported values are: {}",
                        header_name,
                        expected_header_value.to_vec().join(", ")
                    ),
                    ErrorType::InvaidHeaderError,
                )))
            }
            _ => Ok(()),
        }
    }

    pub fn validate_headers(&self, mandatory_headers: &[&str]) -> Result<(), Box<dyn ApiError>> {
        let headers = self.event.extract_headers();
        if mandatory_headers.iter().any(|&h| !headers.contains_key(h)) {
            return Err(Box::new(BadRequestError::new(
                &format!(
                    "Missing one of the mandatory headers: {}",
                    mandatory_headers.to_vec().join(", ")
                ),
                ErrorType::MissingHeaderError,
            )));
        }

        self.validate_header(
            "content-type",
            &["application/json", "application/json; charset=utf-8"],
        )?;

        Ok(())
    }
}
