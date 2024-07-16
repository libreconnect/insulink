use std::{collections::HashMap, error::Error, future::Future, pin::Pin};

use super::Driver;
use reqwest::Client;
use serde_json::Value;

pub struct Medtronic {}

impl Medtronic {
  pub fn new() -> Self {
    Medtronic {}
  }

  async fn connect_impl(
    &self,
    username: &str,
    password: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut credentials = HashMap::new();

    credentials.insert("username", username);
    credentials.insert("password", password);

    let res = client
      .post("https://app.mydiabby.com/api/getToken")
      .json(&credentials)
      .send()
      .await?;

    if res.status().is_success() {
      let token_response: Value = res.json().await?;

      println!("{:?}", token_response);

      Ok(())
    } else {
      Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to connect",
      )))
    }
  }
}

impl Driver for Medtronic {
  fn connect<'a>(
    &'a self,
    username: &'a str,
    password: &'a str,
  ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + 'a>> {
    Box::pin(self.connect_impl(username, password))
  }

  fn get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
    Ok("Data from Medtronic".to_string())
  }
}
