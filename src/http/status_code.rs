use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotAllowed = 405,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

