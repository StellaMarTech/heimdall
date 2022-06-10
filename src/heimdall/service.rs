use crate::errors::HeimdallError;
use crate::mail::MailAccount;

pub enum Endpoint {
    Url(String),
    Command(String),
    LogFile(String),
}

pub enum ServiceStatus {
    On,
    Off,
    Error
}

pub trait Service {

    /// The type returned in if the check function fails
    type CheckErrorType;

    /// The type returned in if the send_alert function fails
    type SendErrorType;

    /// Time interval, in seconds, at which the check function is called.
    fn interval(&self) -> u32;

    fn mail_account(&self) -> &MailAccount;

    fn name(&self) -> &str;

    /// Call the endpoint and return the state of the service depending
    /// on the result of the call. Recommends print a log if the error occurs in the service
    /// and return the error if this occurs in the calling.
    fn check(&self, endpoint: Endpoint) -> Result<ServiceStatus, Self::CheckErrorType>;

    /// It will send an email with the status of the service. It will always be called
    /// after the check function.
    ///
    /// <br>
    ///
    /// ## Params
    /// * service_status: If the result of check function is Self::CheckErrorType, this value will
    /// be None
    /// * mail_account: Always will be the returned value of the mail_account function
    fn send_alert(&self, service_status: Option<ServiceStatus>, mail_account: &MailAccount) -> Result<(), Self::SendErrorType>;
}



pub struct ApiRestService {
    name: String,
    mail_account: MailAccount,
    interval: u32,
}

impl Service for ApiRestService {
    type CheckErrorType = HeimdallError;
    type SendErrorType = HeimdallError;

    fn interval(&self) -> u32 {
        self.interval
    }

    fn mail_account(&self) -> &MailAccount {
        &self.mail_account
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn check(&self, endpoint: Endpoint) -> Result<ServiceStatus, Self::CheckErrorType> {
        todo!()
    }

    fn send_alert(&self, service_status: Option<ServiceStatus>, mail_account: &MailAccount) -> Result<(), Self::SendErrorType> {
        todo!()
    }
}

