
#[derive(Debug)]
pub enum AppError {
    CliConfigServerMissingError(String),
    ConfigIoError(std::io::Error),
    ConfigDeserializationError(serdeconv::Error),
    ConfigSecretEvalCommandError(std::io::Error),
    ConfigSecretEvalBufferReadError(std::string::FromUtf8Error),
    ConfigVarError(std::env::VarError),
    CanvasDeserializeError(serde_json::error::Error),
    // We may wish to get header values from the response, at which point this
    // error is no longer unused.
    #[allow(dead_code)]
    CanvasHeaderError(reqwest::header::ToStrError),
    CanvasRequestError(reqwest::Error),
    LoggingInitializationError(log::SetLoggerError),
}
