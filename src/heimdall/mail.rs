use crate::errors::HeimdallError;
use dotenv::dotenv;

pub struct MailAccount {
    user: String,
    password: String,
    server_url: String,
    to: String,
}

impl MailAccount {

    pub fn new() -> Result<Self, HeimdallError> {
        let _ = dotenv()?;
        
        Ok(Self {
            user: "".to_string(),
            password: "".to_string(),
            server_url: "".to_string(),
            to: "".to_string()
        })
    }

    pub fn send_mail(&self, subject: String, text: String) -> Result<(), HeimdallError> {
        todo!()
    }
}