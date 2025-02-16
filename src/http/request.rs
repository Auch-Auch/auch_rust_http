use std::str;
use std::str::Utf8Error;
use std::option::Option;
use super::method::Method;
use super::method::MethodError;
use super::{Query};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<Query<'buf>>,
    method: Method,
    headers: HashMap<&'buf str, &'buf str>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn headers(&self) -> &HashMap<&str, &str> {
        &self.headers
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(bytes: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(bytes)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        match protocol {
            "HTTP/1.1" => (),
            "HTTP/1.0" => (),
            _ => return Err(ParseError::InvalidProtocol),
        }

        let method: Method = method.parse()?;

        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(Query::from(&path[i + 1..]));
            path = &path[..i];
        }

        let headers = parse_headers(request);

        Ok(Self {
            path,
            query,
            method,
            headers,
        })
    
    }

}
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() { 
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None 
}

fn parse_headers(request: &str) -> HashMap<&str, &str> {

    let mut headers = HashMap::new();
    for line in request.split("\r\n") {
        if line == "" { break; }
        let (key, value) = line.split_once(": ").unwrap();
        headers.insert(key, value);
    }
    headers
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {}