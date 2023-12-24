#![allow(non_snake_case)]
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
pub struct Prediction {
    pub citationMetadata: Citations,
    pub content: String,
    pub safetyAttributes: SafetyAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citations {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citation {
    pub license: String,
    pub uri: String,
    pub title: String,
    pub endIndex: i32,
    pub startIndex: i32,
    pub publicationDate: GoogleTypeDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyAttributes {
    pub scores: Vec<f32>,
    pub blocked: bool,
    pub categories: Vec<String>,
    pub safetyRatings: Vec<SafetyRating>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub severityScore: f32,
    pub severity: String,
    pub probabilityScore: f32,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub tokenMetadata: TokenMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub outputTokenCount: TokenCount,
    pub inputTokenCount: TokenCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCount {
    pub totalBillableCharacters: u32,
    pub totalTokens: u32,
}
