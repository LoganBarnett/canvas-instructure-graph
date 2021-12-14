
#[derive(Debug)]
pub enum AppError {
    CliConfigServerMissingError(String),
    ConfigIoError(std::io::Error),
    ConfigDeserializationError(serdeconv::Error),
    ConfigSecretEvalCommandError(std::io::Error),
    ConfigSecretEvalBufferReadError(std::string::FromUtf8Error),
    ConfigVarError(std::env::VarError),
    CanvasHeaderError(reqwest::header::ToStrError),
    CanvasRequestError(reqwest::Error),
    LoggingInitializationError(log::SetLoggerError),
}
