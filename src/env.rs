use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Env {
    #[clap(env)]
    pub host: String,
    #[clap(env)]
    pub port: u16,
}
