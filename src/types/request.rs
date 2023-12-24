#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub temperature: f32,
    pub maxOutputTokens: u32,
    pub topK: u32,
    pub topP: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VertexApiRequest {
    pub instances: Vec<Instance>,
    pub parameters: Parameters,
}
