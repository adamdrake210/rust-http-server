use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok200 = 200,
  BadRequest400 = 400,
  NotFound404 = 404,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok200 => "OK",
      Self::BadRequest400 => "Bad Request",
      Self::NotFound404 => "Not Found",
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{} {}", *self as u16, self.reason_phrase())
  }
}