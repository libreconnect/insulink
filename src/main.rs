use actix_web::{App, HttpServer};
use clap::Parser;
use dotenv::dotenv;
use env::Env;

mod env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let env = Env::parse();

    let addr_in = format!("{}:{}", env.host, env.port);

    HttpServer::new(move || App::new())
        .bind(addr_in)?
        .run()
        .await?;
    Ok(())
}
