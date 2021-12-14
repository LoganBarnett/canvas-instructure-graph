// Needed to make future magic work, I guess.
use futures::TryFutureExt;

mod canvas;
mod cli;
mod config;
mod logging;
mod error;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let config = config::config_load("canvas-instructure-graph")
        .and_then(cli::cli_validate)?;
    let response = canvas::canvas_request(
        &config,
        reqwest::Method::GET,"https://canvas.instructure.com/api/v1/courses".to_string(),
    )
        .map_err(error::AppError::CanvasRequestError)
        .await?;
    let buffered_response = canvas::to_buffered_response(response)
        .await?;
    println!("{}", buffered_response.text);
    // println!("Done!");
    Ok(())
}
