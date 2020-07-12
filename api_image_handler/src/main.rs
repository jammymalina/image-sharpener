mod domain;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::info;

use domain::{
    api_request::ApiRequest, api_response::ApiResponse, api_validator::ApiValidator,
    errors::api_error::ApiError,
};

fn lambda_handler(event: ApiRequest, _context: Context) -> Result<ApiResponse, HandlerError> {
    let status = execute(event);

    let response = match status {
        Ok(_) => ApiResponse {
            is_base64_encoded: false,
            status_code: 204,
            headers: None,
            body: None,
        },
        Err(err) => err.as_ref().get_http_response(),
    };
    Ok(response)
}

fn execute(event: ApiRequest) -> Result<(), Box<dyn ApiError>> {
    info!("Image operation: {}", event.path_parameters.operation);
    let api_validator = ApiValidator::new(&event);
    api_validator.validate_headers(&["content-type"])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(lambda_handler);
    Ok(())
}
