# Vertex AI and Rust [![Rust](https://github.com/andreban/vertex-api-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/andreban/vertex-api-rust/actions/workflows/rust.yml)

This repository contains a demo for interacting with [Google Cloud Vertex AI](https://cloud.google.com/vertex-ai/docs) services.

## Running the demo

1. Enable the Vertex AI API in the [Cloud Console](https://console.cloud.google.com/apis/library/aiplatform.googleapis.com?q=vertex).
2. Install the [Google Cloud SDK](https://cloud.google.com/sdk/?hl=en_US) is insalled and configured and that you are logged in into an account with the Vertex API enabled in the Cloud Console. You can use `gcloud auth login` to log in.
3. Ensure you have the following environment variables set:
  - `API_ENDPOINT`: The endpoint of the Vertex AI API.
  - `PROJECT_ID`: The ID of the project to use.
  - `LOCATION_ID`: The ID of the location to use (also called region).

Alternatively, create `.cargo/config.toml` with the following content:
```toml
[env]
API_ENDPOINT="<api endpoint>"
PROJECT_ID="<project id>"
LOCATION_ID="<location id (also called region)"
```

4. Then run the demo with `cargo run`.

