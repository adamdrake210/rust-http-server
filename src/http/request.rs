use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /user?id=10 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
      // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
      //   Ok(request) => {},
      //   Err(e) => return Err(e)  
      // }
      //* Below is basically the same as commented out match statement above. 
      let request = str::from_utf8(buf)?;

      let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

      if protocol != "HTTP/1.1" {
        return Err(ParseError::InvalidProtocol);
      }

      let method: Method = method.parse()?;

      let mut query_string = None;
      // match path.find('?') {
      //   Some(i) => {
      //     query_string = Some(&path[i + 1..]);
      //     path = &path[..i];
      //   },
      //   None => {}
      // }
      
      // let q = path.find('?');
      // if q.is_some() {
      //   let i = q.unwrap();
      //   query_string = Some(&path[i + 1..]);
      //   path = &path[..i];
      // }
      //* Below is same as above two commented ways but with if let */
      if let Some(i) = path.find('?') {
        query_string = Some(&path[i + 1..]);
        path = &path[..i];
      }

      unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate()  {
      if c == ' ' || c == '\r' {
        return Some((&request[..i], &request[i + 1..]));
      }
  }

  None
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidEncoding => "Invalid Request",
      Self::InvalidProtocol => "Invalid Protocal",
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
