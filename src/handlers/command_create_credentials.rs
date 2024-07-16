use std::sync::Arc;

use lapin::{message::Delivery, options::BasicAckOptions, Channel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCreateCredentials {
  pub username: String,
  pub password: String,
}

pub async fn create_credentials(
  message: MessageCreateCredentials,
  channel: Arc<Channel>,
  delivery: Delivery,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  println!("Received message: {:?}", message);

  channel
    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
    .await
    .unwrap();
  Ok(())
}
