mod lambda_gateway;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::Deserialize;

use lambda_gateway::{LambdaResponse, LambdaResponseBuilder};

#[derive(Deserialize, Clone)]
struct EmptyEvent {}

fn lambda_handler(_event: EmptyEvent, _context: Context) -> Result<LambdaResponse, HandlerError> {
    let response = LambdaResponseBuilder::new()
        .with_status(204)
        .with_header("Access-Control-Allow-Origin", "*")
        .build();

    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(lambda_handler);
    Ok(())
}
