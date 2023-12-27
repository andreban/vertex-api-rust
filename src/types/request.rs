use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub temperature: f32,
    pub max_output_tokens: u32,
    pub top_k: u32,
    pub top_p: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VertexApiRequest {
    pub instances: Vec<Instance>,
    pub parameters: Parameters,
}
