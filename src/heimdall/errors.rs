use std::fmt::{Display, Formatter, Result};
use heimdall_errors::implement_error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    Dotenv,
    Env,
    Io,
    JWT,
    Serde,
    Chrono,
    Lettre,
    Email,
    Internal,
    Input,
    Unknown,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeimdallError {
    kind: ErrorKind,
    message: String,
}

impl HeimdallError {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for HeimdallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "kind: {} message: {}",
            self.kind.to_string(),
            self.message
        )
    }
}
implement_error!(HeimdallError, dotenv::Error, ErrorKind::Dotenv);
implement_error!(HeimdallError, std::io::Error, ErrorKind::Io);
