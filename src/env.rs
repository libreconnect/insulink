use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Env {
  #[clap(env)]
  pub host: String,
  #[clap(env)]
  pub port: u16,
  #[clap(env)]
  pub workers: usize,

  #[clap(env)]
  pub rabbitmq_url: String,
  #[clap(env)]
  pub rabbitmq_password: String,
  #[clap(env)]
  pub rabbitmq_user: String,
  #[clap(env)]
  pub rabbitmq_port: u16,

  #[clap(env)]
  pub postgres_url: String,
  #[clap(env)]
  pub postgres_password: String,
  #[clap(env)]
  pub postgres_user: String,
  #[clap(env)]
  pub postgres_port: u16,
  #[clap(env)]
  pub postgres_db: String,

  #[clap(env)]
  pub medtronic_username: String,
  #[clap(env)]
  pub medtronic_password: String,
}
