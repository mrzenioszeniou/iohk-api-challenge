use std::fmt::Display;
use std::{error::Error as ErrorTrait, fmt::Debug};

pub struct Error {
  message: String,
}

impl From<&str> for Error {
  fn from(from: &str) -> Self {
    Self {
      message: String::from(from),
    }
  }
}

impl From<String> for Error {
  fn from(message: String) -> Self {
    Self { message }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl ErrorTrait for Error {}
