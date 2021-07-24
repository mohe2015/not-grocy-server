use std::fmt;
use thiserror::Error;

// TODO FIXME improve

#[derive(Error, Debug)]
pub struct DieselError(pub diesel::result::Error);

impl fmt::Display for DieselError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
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

impl actix_web::error::ResponseError for R2D2Error {}
