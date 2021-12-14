// Give me all of the futures magic, reasonability be damned.
use futures::FutureExt;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use reqwest;

use crate::cli;
use crate::error;

// Responses are consumed the moment you read in something like its body. If
// easily toggleable debugging is desired, reqwest::Response is not the way to
// go - you will enter borrow-checker hell, from which there is no escape or
// respite. This is proven science.
//
#[derive(Debug)]
pub struct BufferedResponse {
    pub headers: reqwest::header::HeaderMap,
    pub status: reqwest::StatusCode,
    pub text: String,
}

/// Make a generic request to the Canvas API using the auth token.
pub async fn canvas_request<'a>(
    config: &'a cli::CliValid,
    method: reqwest::Method,
    url: String,
) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .request(method, url)
        .bearer_auth(config.server.api_token.clone())
        .send()
        .await
}

pub async fn to_buffered_response(
    r: reqwest::Response,
) -> Result<BufferedResponse, error::AppError> {
    Ok(BufferedResponse {
        headers: r.headers().clone(),
        status: r.status(),
        // TODO: Make this error more appropriate.
        text: r.text().await.map_err(error::AppError::CanvasRequestError)?,
    })
}
