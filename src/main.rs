use std::sync::Arc;

use ::lapin::{auth::Credentials, Channel};
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use database::Database;
use dotenv::dotenv;
use env::Env;
use handlers::listen_queue;
use health::{health_controller, health_service::HealthService};
use lapin::LapinClient;
use sqlx::PgPool;
use tracing::info;

mod database;
mod drivers;
mod env;
mod handlers;
mod health;
mod lapin;

async fn spawn_listener(
  queue_name: String,
  channel: Arc<Channel>,
  pool: Arc<PgPool>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let pool_clone = Arc::clone(&pool);
  let channel_clone = channel.clone();
  tokio::spawn(async move {
    let _ = listen_queue(&queue_name, channel_clone.clone(), pool_clone.clone()).await;
  });

  Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt::init();
  dotenv().ok();
  let env = Env::parse();

  let addr_in = format!("{}:{}", env.host, env.port);
  let database = Database::new(
    env.postgres_user,
    env.postgres_password,
    env.postgres_url,
    env.postgres_port,
    env.postgres_db,
  )
  .await;

  let pool = Arc::new(database.pool);

  let creds = Credentials::new(env.rabbitmq_user, env.rabbitmq_password);
  let lapin = LapinClient::new(env.rabbitmq_url, env.rabbitmq_port, creds).await?;

  lapin.configure_service().await?;

  info!("Starting server at: {}", addr_in);

  spawn_listener(
    "insulink_fetch_data".to_string(),
    lapin.channel.clone(),
    pool.clone(),
  )
  .await
  .unwrap();

  let health_service = Arc::new(HealthService::new(pool.clone()));

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(health_service.clone()))
      .service(health_controller::live)
      .service(health_controller::readiness)
  })
  .bind(addr_in)?
  .workers(env.workers)
  .run()
  .await?;

  lapin.close().await;
  Ok(())
}
