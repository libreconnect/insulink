use std::sync::Arc;

use lapin::{message::Delivery, options::BasicAckOptions, Channel};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::drivers::{medtronic::Medtronic, Driver};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
  start_date: Option<String>,
  end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFetchOnce {
  user_id: String,
  meta: Meta,
}

pub async fn fetch_once(
  message: MessageFetchOnce,
  channel: Arc<Channel>,
  delivery: Delivery,
  pool: Arc<PgPool>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  println!("Received message: {:?}", message);

  //let medtronic = Medtronic::new();

  channel
    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
    .await
    .unwrap();

  Ok(())
}
