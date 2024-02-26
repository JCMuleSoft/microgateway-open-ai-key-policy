use anyhow::Result;
use pdk::api::hl::*;

use crate::generated::config::Config;

// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;


// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter<'a>(
    request_state: RequestState,
    _http_client: &HttpClient,
    config: &'a Config,
) -> Flow<()> {
    let vec = config.clone().api_key;
    let index = (rand::random::<f32>() * vec.len() as f32).floor() as usize;
    let api_key = vec.get(index).unwrap();
    let headers = request_state.into_headers_state().await;
    headers.add_header("Authorization", format!("Bearer {}", api_key).as_str());
    Flow::Continue(())
}

#[entrypoint]
async fn configure(
    launcher: Launcher,
    Configuration(bytes): Configuration,
    http_client: HttpClient,
) -> Result<()> {
    let config = serde_json::from_slice(&bytes)?;
    let filter = on_request(|rs| request_filter(rs, &http_client, &config));
    launcher.launch(filter).await?;
    Ok(())
}
