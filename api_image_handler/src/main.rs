mod domain;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::info;

use domain::{api_request::ApiRequest, api_response::ApiResponse};

fn lambda_handler(event: ApiRequest, _context: Context) -> Result<ApiResponse, HandlerError> {
    info!("Image operation: {}", event.path_parameters.operation);
    let response = ApiResponse {
        is_base64_encoded: false,
        status_code: 204,
        headers: None,
        body: None,
    };
    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(lambda_handler);
    Ok(())
}
