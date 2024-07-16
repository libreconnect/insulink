use std::sync::Arc;

use lapin::{
  auth::Credentials,
  options::{ExchangeDeclareOptions, QueueDeclareOptions},
  types::{AMQPValue, FieldTable},
  Channel, Connection, ConnectionProperties, ExchangeKind, Queue,
};

pub struct LapinClient {
  pub conn: Connection,
  pub channel: Arc<Channel>,
}

impl LapinClient {
  pub async fn new(host: String, port: u16, creds: Credentials) -> lapin::Result<Self> {
    let uri = format!(
      "amqp://{}:{}@{}:{}/%2f",
      creds.username(),
      creds.password(),
      host,
      port
    );

    println!("{}", uri);

    let conn = Connection::connect(&uri, ConnectionProperties::default()).await?;

    println!("Connected to RabbitMQ");
    let channel = conn.create_channel().await?;
    let channel = Arc::new(channel);

    Ok(Self { conn, channel })
  }

  pub async fn close(&self) {
    self.conn.close(200, "Goodbye").await.unwrap();
  }

  pub async fn declare_exchange(&self, name: &str, kind: ExchangeKind) -> lapin::Result<()> {
    self
      .channel
      .exchange_declare(
        name,
        kind,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
      )
      .await
  }

  pub async fn declare_queue(&self, name: &str, durable: bool) -> lapin::Result<Queue> {
    let mut arguments = FieldTable::default();

    arguments.insert(
      "x-queue-type".into(),
      AMQPValue::LongString("quorum".into()),
    );

    self
      .channel
      .queue_declare(
        name,
        QueueDeclareOptions {
          durable,
          ..Default::default()
        },
        arguments,
      )
      .await
  }

  pub async fn configure_service(&self) -> Result<(), Box<dyn std::error::Error>> {
    let exchange_name = "insulink_topic_exchange";
    self
      .declare_exchange(&exchange_name, ExchangeKind::Topic)
      .await?;

    self.declare_queue("insulink_credentials", true).await?;
    self.declare_queue("insulink_fetch_data", true).await?;

    self
      .channel
      .queue_bind(
        "insulink_credentials",
        &exchange_name,
        "insulink.credentials.v1",
        Default::default(),
        Default::default(),
      )
      .await?;

    self
      .channel
      .queue_bind(
        "insulink_fetch_data",
        &exchange_name,
        "insulink.fetch_data.v1",
        Default::default(),
        Default::default(),
      )
      .await?;
    Ok(())
  }
}
