///
/// This file contains the structs that represent the response from the Vertex AI API.
/// Based on the data at https://aiplatform.googleapis.com/$discovery/rest?version=v1.
///
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VertexApiResponse {
    pub predictions: Vec<Prediction>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prediction {
    pub citation_metadata: Citations,
    pub content: String,
    pub safety_attributes: SafetyAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citations {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    pub license: String,
    pub uri: String,
    pub title: String,
    pub end_index: i32,
    pub start_index: i32,
    pub publication_date: GoogleTypeDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyAttributes {
    pub scores: Vec<f32>,
    pub blocked: bool,
    pub categories: Vec<String>,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub severity_score: f32,
    pub severity: String,
    pub probability_score: f32,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub token_metadata: TokenMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenMetadata {
    pub output_token_count: TokenCount,
    pub input_token_count: TokenCount,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCount {
    pub total_billable_characters: u32,
    pub total_tokens: u32,
}
