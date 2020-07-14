use super::errors::{
    api_error::ApiError, bad_request_error::BadRequestError, error_type::ErrorType,
};
use super::{api_request::ApiRequest, schedule_image_op_request::ScheduleImageOpBody};

pub struct ApiValidator {
    event: ApiRequest,
}

impl ApiValidator {
    pub fn new(event: &ApiRequest) -> Self {
        ApiValidator {
            event: event.clone(),
        }
    }

    fn validate_base64_str(base64_str: &str) -> Result<(), Box<dyn ApiError>> {
        let base64_char_count: usize = (b'a'..b'z')
            .chain(b'A'..b'Z')
            .chain(b'0'..b'9')
            .chain(vec![b'+', b'/', b'='].into_iter())
            .map(|base64_char| base64_str.matches(char::from(base64_char)).count())
            .sum();
        if base64_char_count < base64_str.len() {
            return Err(Box::new(BadRequestError::new(
                "Invalid body",
                ErrorType::InvalidBodyError,
            )));
        }
        Ok(())
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
                    ErrorType::InvalidHeaderError,
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

    pub fn validate_body(&self, body: &ScheduleImageOpBody) -> Result<(), Box<dyn ApiError>> {
        Self::validate_base64_str(&body.image_base64)?;
        Ok(())
    }
}
