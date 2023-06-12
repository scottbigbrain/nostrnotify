use std::time::SystemTime;
use time::OffsetDateTime;
use nostr_sdk::EventId;
use clap::{ Parser, Subcommand };

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
    SetMetadata,
    SecretKey { secret_key: String },
    FeedURL { feed_url: String },
    Interval { interval: u64 },
    AddRelay { relay: String },
}

pub fn print_check_log(feed_url: &String) {
    let check_timestamp: OffsetDateTime = SystemTime::now().into();
    println!("Requested {feed_url} at {check_timestamp}");
}

pub fn print_notify_log(event_id: EventId) {
    let notifcation_timestamp: OffsetDateTime = SystemTime::now().into();
    println!("Broadcasted new episode notifcation at {notifcation_timestamp} with {event_id}")
}
