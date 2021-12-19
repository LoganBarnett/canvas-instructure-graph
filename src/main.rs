mod canvas;
mod cli;
mod config;
mod error;
mod http;
mod logging;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let config = config::config_load("canvas-instructure-graph")
        .and_then(cli::cli_validate)?;
    let courses = canvas::courses(&config).await?;
    println!("{:#?}", courses);
    // println!("Done!");
    Ok(())
}
