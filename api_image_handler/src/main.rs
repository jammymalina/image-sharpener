mod domain;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::info;

use domain::{
    api_gateway_mapper::ApiGatewayMappper, api_request::ApiRequest, api_response::ApiResponse,
    api_response::ApiResponseBuilder, api_validator::ApiValidator, errors::api_error::ApiError,
};

fn lambda_handler(event: ApiRequest, _context: Context) -> Result<ApiResponse, HandlerError> {
    let status = handle_request(&event);

    let response = match status {
        Ok(_) => wrap_response(
            ApiResponse {
                is_base64_encoded: false,
                status_code: 204,
                headers: None,
                body: None,
            },
            &event,
        ),
        Err(err) => wrap_response(err.as_ref().get_http_response(), &event),
    };
    Ok(response)
}

fn wrap_response(response: ApiResponse, event: &ApiRequest) -> ApiResponse {
    let request_headers = event.extract_headers();
    let mut response_builder = ApiResponseBuilder::from_response(response);

    if request_headers.contains_key("user-agent") {
        response_builder = response_builder.with_header("Access-Control-Allow-Origin", "*");
    }

    response_builder.build()
}

fn handle_request(event: &ApiRequest) -> Result<(), Box<dyn ApiError>> {
    info!("Image operation: {}", event.path_parameters.operation);
    let api_validator = ApiValidator::new(&event);
    let api_gw_mapper = ApiGatewayMappper::new(&event);
    let body = api_gw_mapper.parse_body()?;

    api_validator.validate_headers(&["content-type"])?;
    api_validator.validate_body(&body)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(lambda_handler);
    Ok(())
}
