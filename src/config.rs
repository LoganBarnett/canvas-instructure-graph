use crate::error;
use serde::{Deserialize, Serialize};
use serde;
use serdeconv;
use std::collections::{HashMap};
use std::env;
use std::fs;
use std::process::Command;


#[derive(Serialize, Deserialize)]
pub struct ConfigFromFile {
    pub default_server: String,
    #[serde(flatten)]
    pub servers: HashMap<String, ConfigServerFromFile>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigServerFromFile {
    pub host_url: String,
     /// The string to be evaluated using the shell which will provide the token.
     /// If you feel comfortable leaving your security token in here directly,
     /// just surround it with single quotes, for example: "'my-token'"
    pub token_eval: String,
}

#[derive(Clone)]
pub struct ConfigParsed {
    pub default_server: String,
    pub servers: HashMap<String, ConfigServerParsed>,
}

#[derive(Clone)]
pub struct ConfigServerParsed {
    pub name: String,
    pub host_url: String,
    pub api_token: String,
}

fn config_dir_ensure(app_name: &str) -> Result<(), error::AppError> {
    fs::create_dir_all(
        path(&[
            &env::var("HOME").map_err(error::AppError::ConfigVarError)?,
            &".config".to_string(),
            &app_name.to_string(),
        ])
    ).map_err(error::AppError::ConfigIoError)
}


fn config_from_file(app_name: &str) -> Result<ConfigFromFile, error::AppError> {
    serdeconv::from_toml_file(
        path(&[
            &env::var("HOME").map_err(error::AppError::ConfigVarError)?,
            &".config".to_string(),
            &app_name.to_string(),
            &"config.toml".to_string(),
        ])
    ).map_err(error::AppError::ConfigDeserializationError)
}

pub fn config_load(app_name: &str) -> Result<ConfigParsed, error::AppError> {
    config_dir_ensure(app_name)?;
    config_from_file(app_name)
        .and_then(config_validate)
}

// defaultServer should exist among servers, or something is wrong.
fn config_validate(
    config_from_file: ConfigFromFile,
) -> Result<ConfigParsed, error::AppError> {
    Ok(ConfigParsed {
        default_server: config_from_file.default_server,
        servers: config_from_file.servers.into_iter().map(|(k, v)| {
            Ok((k.clone(), ConfigServerParsed {
                name: k,
                host_url: v.host_url,
                api_token: secret_eval(v.token_eval)?,
            }))
        }).collect::<Result<
                HashMap<String, ConfigServerParsed>,
                error::AppError,
            >>()?,
    })
}

fn path(paths: &[&str]) -> std::path::PathBuf {
    paths.iter().collect()
}

/// Evaluate a shell expression to generate a secret value.
fn secret_eval(secret_code: String) -> Result<String, error::AppError> {
    // Beware that sh could be a shell you don't exepct in your environment..
    Command::new("sh")
        .args(&["-c", &secret_code])
        .output()
        .map_err(error::AppError::ConfigSecretEvalCommandError)
        .and_then(|x| {
            String::from_utf8(x.stdout)
                  .map_err(error::AppError::ConfigSecretEvalBufferReadError)
        })
        .map(|x| x.to_string())
}
