use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run,
    GenerateKeys,
    SecretKey { secret_key: String },
    FeedURL { feed_url: String },
    Interval { interval: u64 },
    AddRelay { relay: String },
}
