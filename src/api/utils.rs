use std::fmt;

use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};

// TODO FIXME improve

#[derive(Debug)]
pub struct DieselError(pub diesel::result::Error);

impl fmt::Display for DieselError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<DieselError> for std::io::Error {
    fn from(error: DieselError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl actix_web::error::ResponseError for DieselError {}

#[derive(Debug)]
pub struct R2D2Error(pub r2d2::Error);

impl fmt::Display for R2D2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<R2D2Error> for std::io::Error {
    fn from(error: R2D2Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl actix_web::error::ResponseError for R2D2Error {}

#[derive(Debug)]
pub struct OAuthError(
    pub  RequestTokenError<
        oauth2::reqwest::Error<reqwest::Error>,
        StandardErrorResponse<BasicErrorResponseType>,
    >,
);

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{:?}", self.0);
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<OAuthError> for std::io::Error {
    fn from(error: OAuthError) -> Self {
        println!("{:?}", error);
        std::io::Error::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl actix_web::error::ResponseError for OAuthError {}
