// Needed to make future magic work, I guess.
use futures::TryFutureExt;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::cli;
use crate::error;

// Responses are consumed the moment you read in something like its body. If
// easily toggleable debugging is desired, reqwest::Response is not the way to
// go - you will enter borrow-checker hell, from which there is no escape or
// respite. This is proven science.
#[derive(Debug)]
pub struct BufferedResponse {
    pub headers: reqwest::header::HeaderMap,
    pub status: reqwest::StatusCode,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CanvasErrorResponse {
    pub errors: Vec<CanvasError>,
    pub error_report_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CanvasError {
    pub message: String,
    pub error_code: String,
}

/// Make a generic request and deserialize the response.
pub async fn request<'a, A: serde::de::DeserializeOwned>(
    config: &'a cli::CliValid,
    method: reqwest::Method,
    url: String,
  ) -> Result<A, error::AppError> {
    let buffered_response = request_buffered(
        config,
        method,
        url,
    ).await?;
    // If there is a server error, there should be an accompanying payload we
    // can inspect.
    if buffered_response.status.as_u16() < 400 {
        serde_json::from_str::<A>(&buffered_response.text)
            .map_err(error::AppError::CanvasDeserializeError)
    } else {
        let error = serde_json::from_str::<CanvasErrorResponse>(
            &buffered_response.text,
        )
            .map_err(error::AppError::CanvasDeserializeError)?;
        Err(error::AppError::CanvasServerError(error))
    }
}

/// Make a generic request to the Canvas API using the auth token.
pub async fn request_raw<'a>(
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

pub async fn request_buffered<'a>(
    config: &'a cli::CliValid,
    method: reqwest::Method,
    url: String,
) -> Result<BufferedResponse, error::AppError> {
    let response = request_raw(config, method.clone(), url.clone())
        .map_err(error::AppError::CanvasRequestError)
        .await?;
    let buffered_response = to_buffered_response(response).await?;
    log::debug!(
        "Response from {} {}: {:#?}",
        method,
        url,
        buffered_response,
    );
    Ok(buffered_response)
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
