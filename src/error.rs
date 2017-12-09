#[cfg(feature = "reqwest")]
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[cfg(feature = "hyper")]
use hyper::error::UriError;
#[cfg(feature = "reqwest")]
use reqwest::{
    Error as ReqwestError,
    Response as ReqwestResponse,
    UrlError as ReqwestUrlError,
};

/// A result type to compose a successful value and the library's [`Error`]
/// type.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = StdResult<T, Error>;

/// An error type to compose a singular error enum between various dependencies'
/// errors.
#[derive(Debug)]
pub enum Error {
    /// An error from the `serde_json` crate.
    ///
    /// A potential reason for this is when there is an error deserializing a
    /// JSON response body.
    #[cfg(feature = "reqwest")]
    Json(JsonError),
    /// An error from the `reqwest` crate when it is enabled.
    #[cfg(feature = "reqwest")]
    Reqwest(ReqwestError),
    /// An error indicating a bad request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestBad(Box<ReqwestResponse>),
    /// An error indicating an invalid request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestInvalid(Box<ReqwestResponse>),
    /// An error indicating a parsing issue when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestParse(ReqwestUrlError),
    /// An error indicating an unathorized request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestUnauthorized(Box<ReqwestResponse>),
    /// An error when building a request's URI from the `hyper` crate when it is
    /// enabled.
    #[cfg(feature = "hyper")]
    Uri(UriError),
}

#[cfg(feature = "reqwest")]
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestUrlError> for Error {
    fn from(err: ReqwestUrlError) -> Self {
        Error::ReqwestParse(err)
    }
}

#[cfg(feature = "hyper")]
impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Error::Uri(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "reqwest")]
            Error::Json(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::ReqwestBad(_) => "Request bad",
            #[cfg(feature = "reqwest")]
            Error::ReqwestInvalid(_) => "Request invalid",
            #[cfg(feature = "reqwest")]
            Error::ReqwestParse(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::ReqwestUnauthorized(_) => "Request auth bad",
            #[cfg(feature = "hyper")]
            Error::Uri(ref inner) => inner.description(),
        }
    }
}
