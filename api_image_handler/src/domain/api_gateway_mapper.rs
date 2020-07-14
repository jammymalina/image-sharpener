use super::errors::{
    api_error::ApiError, bad_request_error::BadRequestError, error_type::ErrorType,
};
use super::{api_request::ApiRequest, schedule_image_op_request::ScheduleImageOpBody};

pub struct ApiGatewayMappper {
    event: ApiRequest,
}

impl ApiGatewayMappper {
    pub fn new(event: &ApiRequest) -> Self {
        ApiGatewayMappper {
            event: event.clone(),
        }
    }

    pub fn parse_body(&self) -> Result<ScheduleImageOpBody, Box<dyn ApiError>> {
        let default_body = String::from("");
        let body = self.event.body.as_ref().unwrap_or(&default_body);
        match serde_json::from_str(body) {
            Ok(req_body) => Ok(req_body),
            _ => Err(Box::new(BadRequestError::new(
                "Invalid body",
                ErrorType::InvalidBodyError,
            ))),
        }
    }
}
