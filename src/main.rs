use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    height: f32, // in meters
    weight: f32, // in kilograms
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    bmi: f32,
    category: String,
}

fn calculate_bmi(height: f32, weight: f32) -> f32 {
    weight / (height * height)
}

fn get_bmi_category(bmi: f32) -> String {
    match bmi {
        bmi if bmi < 18.5 => "Underweight",
        bmi if bmi < 25.0 => "Normal weight",
        bmi if bmi < 30.0 => "Overweight",
        _ => "Obese",
    }
    .to_string()
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let height = event.payload.height;
    let weight = event.payload.weight;

    let bmi = calculate_bmi(height, weight);
    let category = get_bmi_category(bmi);

    let resp = Response {
        req_id: event.context.request_id,
        bmi,
        category,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
