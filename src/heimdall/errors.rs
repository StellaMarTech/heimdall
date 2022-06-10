use std::fmt::{Display, Formatter, Result};

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

mod errors_implementation {
    use crate::implement_error;
    use super::HeimdallError;
    use super::ErrorKind::{
        Dotenv,
        Io,
    };
    use dotenv::Error as dotenvError;
    use std::io::Error as ioError;




    implement_error!(HeimdallError, dotenvError, Dotenv);
    implement_error!(HeimdallError, ioError, Io);

}
