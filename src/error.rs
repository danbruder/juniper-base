//
// error.rs
//
use std::fmt::{Display,Formatter,Result};
use validator::ValidationErrors;
use std::error::Error;
use self::Error::*;

#[derive(Debug)]
pub enum Error {
    DieselError(diesel::result::Error),
    ValidationError,

    // Access
    AccessDenied,

}

pub struct ValidationError{
    field: String,
    message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result{
        match self {
            AccessDenied => write!(f, "Access denied"),
            ValidationError => write!(f, "Error"),
            DieselError(ref e) => write!(f, "{}", e.description()),
        }
    }
}

pub fn from_validation_errors(e: ValidationErrors) -> Vec<ValidationError> {
    let field_errors = e.field_errors();
    field_errors.iter().map(|(k, v)| {
        let messages = v
            .into_iter()
            .filter(|f| f.message.is_some())
            .map(|f| f.clone().message.unwrap().to_string())
            .collect::<Vec<String>>().join(", ");
        ValidationError{
            field: k.to_string(),
            message: messages
        }
    }).collect::<Vec<ValidationError>>()
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        DieselError(error)
    }
}
