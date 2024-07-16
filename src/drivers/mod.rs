use std::{error::Error, future::Future, pin::Pin};

pub mod medtronic;

pub trait Driver {
  fn get_data(&self) -> Result<String, Box<dyn std::error::Error>>;
  fn connect<'a>(
    &'a self,
    username: &'a str,
    password: &'a str,
  ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + 'a>>;
}
