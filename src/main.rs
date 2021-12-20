mod canvas;
mod cli;
mod config;
mod error;
mod http;
mod logging;

use futures::future::try_join_all;
use partial_application::partial;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let config = config::config_load("canvas-instructure-graph")
        .and_then(cli::cli_validate)?;
    let courses = canvas::courses(&config).await?;
    let enrollment_futures = courses
        .into_iter()
        .map(|c| { canvas::enrollments_by_course(&config, c.id) });
    let enrollments = try_join_all(enrollment_futures)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<canvas::Enrollment>>()
    ;
    println!("{:#?}", enrollments);
    Ok(())
}
