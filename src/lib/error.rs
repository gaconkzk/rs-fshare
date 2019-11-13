use std::convert::From;
use std::fmt:: {Display, Formatter, Result};

use std::error::Error as StdError;

use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
  NotVipAccount,
  CsrfNotFound,
  ReqwestError(String),
}

impl From<ReqwestError> for Error {
  fn from(e: ReqwestError) -> Self {
    return Error::ReqwestError(String::from(e.description()))
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match &self {
      Error::NotVipAccount => f.write_str("You are not VIP"),
      Error::CsrfNotFound => f.write_str("The CSRF token not found"),
      Error::ReqwestError(desc) => f.write_str(desc),
    }
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    "deprecated"
  }
}
