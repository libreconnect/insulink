use command_create_credentials::{create_credentials, MessageCreateCredentials};
use command_fetch::{fetch_once, MessageFetchOnce};
use futures_lite::StreamExt;
use lapin::{
  options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
  types::{AMQPValue, FieldTable},
  Channel,
};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

pub mod command_create_credentials;
mod command_fetch;

pub async fn listen_queue(
  queue_name: &str,
  channel: Arc<Channel>,
  pool: Arc<PgPool>,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut arguments = FieldTable::default();

  arguments.insert(
    "x-queue-type".into(),
    AMQPValue::LongString("quorum".into()),
  );
  let queue = channel
    .queue_declare(
      queue_name,
      QueueDeclareOptions {
        durable: true,
        ..Default::default()
      },
      arguments,
    )
    .await;

  if let Err(err) = queue {
    println!("Error declaring queue: {:?}", err);
    return Err(Box::new(err));
  }

  let mut consumer = channel
    .basic_consume(
      queue_name,
      queue_name,
      BasicConsumeOptions {
        no_ack: false,
        ..Default::default()
      },
      Default::default(),
    )
    .await
    .unwrap();

  info!("Listening to queue: {}", queue_name);

  while let Some(delivery) = consumer.next().await {
    let delivery = delivery.unwrap();
    let message = &delivery.data;

    let message = std::str::from_utf8(message).unwrap();

    info!("Received message: {}", message);

    match queue_name {
      "insulink_fetch_data" => {
        let message = serde_json::from_str::<MessageFetchOnce>(message);

        if let Ok(message) = message {
          fetch_once(message, Arc::clone(&channel), delivery, Arc::clone(&pool))
            .await
            .unwrap();
          // let channel = channel.clone();
          // let pool = Arc::new(Mutex::new(pool.))
          // //fetch_once(message, channel.clone(), delivery, pool.clone()).await.unwrap();
          // tokio::spawn(async move {
          //   fetch_once(message, channel.clone(), delivery, pool.clone()).await.unwrap();
          // });
        } else {
          println!("Error parsing message: {:?}", message);
          channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .unwrap();
        }
      }
      "insulink_credentials" => {
        let message = serde_json::from_str::<MessageCreateCredentials>(message);

        if let Ok(message) = message {
          create_credentials(message, channel.clone(), delivery)
            .await
            .unwrap();
        } else {
          println!("Error parsing message: {:?}", message);
          channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .unwrap();
        }
        // get type in message and call the appropriate handler
        // let command_type = serde_json::from_str::<serde_json::Value>(message).unwrap();

        // match command_type["type"].as_str().unwrap() {
        //     "fetch_once" => {
        //         let message = serde_json::from_str::<MessageFetchOnce>(message).unwrap();
        //         // call fetch_once handler
        //         fetch_once(message, channel.clone(), delivery).await.unwrap();
        //     }
        //     _ => (),
        // };
      }
      _ => (),
    };
  }
  Ok(())
}
