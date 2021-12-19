use clap::Parser;
use crate::config;
use crate::error;
use crate::logging;

/// The various inputs aggregated into one place - not yet validated.
#[derive(Parser, Debug)]
#[clap(
    name = "canvas-instructure-graph",
    about = "Create a graph from Instructure's Canvas HTTP API.",
)]
#[clap()]
// Without a structopt declaration, the argument is positional.
pub struct CliInput {
    #[clap(short = 'a', long)]
    pub api_token: Option<String>,
    #[clap(default_value = "default", short, long)]
    pub server: String,
    #[clap(long, short = 'v', parse(from_occurrences))]
    pub verbosity: usize,
}

pub struct CliValid {
    pub verbosity: usize,
    pub server: config::ConfigServerParsed,
}

pub fn cli_validate(
    config: config::ConfigParsed,
) -> Result<CliValid, error::AppError> {
    let cli = CliInput::parse();
    logging::init_logger(cli.verbosity)?;
    let server_name = if cli.server == "default" {
        config.default_server
    } else {
        cli.server
    };
    match config.servers.get(&server_name) {
        Some(server) => Ok(CliValid {
            server: server.clone(),
            verbosity: cli.verbosity,
        }),
        None => Err(error::AppError::CliConfigServerMissingError(
            format!(
                "Could not find server '{}' in configuration.",
                server_name,
            ),
        )),
    }
}
