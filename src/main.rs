///
/// A sample application that demonstrates how to use the Vertex AI API to make predictions.
///
use std::env;

use gcp_auth::AuthenticationManager;

mod types;
use types::request::{Instance, Parameters, VertexApiRequest};

static MODEL_NAME: &str = "text-bison";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let predict_url = format!(
        "https://{api_endpoint}/v1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:predict"
    );

    let resp = reqwest::Client::new()
        .post(&predict_url)
        .bearer_auth(token.as_str())
        .json(&VertexApiRequest {
            instances: vec![Instance {
                prompt: "Give me ten interview questions for the role of program manager."
                    .to_string(),
            }],
            parameters: Parameters {
                temperature: 0.5,
                maxOutputTokens: 100,
                topK: 0,
                topP: 1.0,
            },
        })
        .send()
        .await?;

    let response = resp.json::<types::response::VertexApiResponse>().await?;
    println!("Predictions: {:#?}", response.predictions);
    Ok(())
}
