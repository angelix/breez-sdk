use reqwest::StatusCode;
use std::time::Duration;

pub type SdkResult<T, E = ExternalSdkError> = anyhow::Result<T, E>;

// TODO Rename to SdkError once existing SDKError struct is replaced
/// Public error kinds returned by the SDK
#[derive(Debug)]
pub enum ExternalSdkError {
    LnInvoiceError { err: String },

    // TODO Generic type, contains all new and unknown errors
    Generic,
}

/// Conversion from granular internal errors to external user-facing SDK Errors
impl From<InternalSdkError> for ExternalSdkError {
    fn from(value: InternalSdkError) -> Self {
        match value {
            InternalSdkError::LnInvoiceError(err) => Self::LnInvoiceError { err },
            _ => Self::Generic,
        }
    }
}

pub(crate) type InternalSdkResult<T, E = InternalSdkError> = anyhow::Result<T, E>;

/// Error kinds thrown internally by SDK modules
#[derive(Debug)]
pub(crate) enum InternalSdkError {
    /// Error with the parsing or the validation of an invoice
    LnInvoiceError(String),

    LnUrlPayValidationError(String),
    LnUrlPayValidationErrorIVDecode(String),

    LnUrlWithdrawValidationError(String),
    LnUrlWithdrawCallbackParsingError(String),

    HttpsConnectionError(Option<StatusCode>, String),

    /// Caller tried to determine duration where `from` is after `to`
    SystemTimeError(Duration),
}

impl From<lightning_invoice::ParseError> for InternalSdkError {
    fn from(value: lightning_invoice::ParseError) -> Self {
        Self::LnInvoiceError(value.to_string())
    }
}

impl From<lightning_invoice::SemanticError> for InternalSdkError {
    fn from(value: lightning_invoice::SemanticError) -> Self {
        Self::LnInvoiceError(value.to_string())
    }
}

impl From<reqwest::Error> for InternalSdkError {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpsConnectionError(value.status(), value.to_string())
    }
}

impl From<std::time::SystemTimeError> for InternalSdkError {
    fn from(value: std::time::SystemTimeError) -> Self {
        todo!()
    }
}

impl From<url::ParseError> for InternalSdkError {
    fn from(value: url::ParseError) -> Self {
        todo!()
    }
}
