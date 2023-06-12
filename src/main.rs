use std::{
    self,
    error::Error,
    time::Duration,
};
use nostr_sdk::prelude::*;
use rss::Channel;
use reqwest;
use tokio;
use confy;
use clap::Parser;
use crate::config::*;
use crate::cli::*;
use crate::notification::*;

pub mod config;
pub mod cli;
pub mod notification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut cfg: Config = confy::load("nostrnotify", None)?;
    
    match &cli.command {
        Commands::Run => monitor_mode(cfg).await?,
        Commands::GenerateKeys => generate_keys(cfg)?,
        Commands::SetMetadata => request_user_metadata(cfg)?,
        Commands::SecretKey { secret_key } => {
            cfg.secret_key = String::from(secret_key);
            cfg.public_key = Keys::from_sk_str(secret_key).unwrap().public_key().to_bech32().unwrap();
            confy::store("nostrnotify", None, cfg)?
        },
        Commands::FeedURL { feed_url } => {
            cfg.feed_url = String::from(feed_url);
            confy::store("nostrnotify", None, cfg)?
        },
        Commands::Interval { interval } => {
            cfg.check_interval_seconds = *interval;
            confy::store("nostrnotify", None, cfg)?
        },
        Commands::AddRelay { relay } => {
            cfg.relays.push(String::from(relay));
            confy::store("nostrnotify", None, cfg)?
        },
    }

    Ok(())
}

async fn monitor_mode(cfg: Config) -> Result<(), Box<dyn Error>> {
    let my_keys = Keys::from_sk_str(&cfg.secret_key).unwrap();
    let client = Client::new(&my_keys);
    
    for relay in cfg.relays {
        client.add_relay(&relay, None).await?;
    }
    client.connect().await;
    
    let metadata = Metadata::new()
        .name(&cfg.name)
        .display_name(&cfg.display_name)
        .about(&cfg.description);
    client.set_metadata(metadata).await?;
    
    let feed = get_feed(&cfg.feed_url).await?;
    let mut last_feed_len = feed.items.len();
    
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.check_interval_seconds));
    
    loop {
        interval.tick().await;
        let feed = get_feed(&cfg.feed_url).await?;
        print_check_log(&cfg.feed_url);
        if feed.items.len() > last_feed_len {
            last_feed_len = feed.items.len();
            let event_id = publish_notification(&feed, &client).await?;
            print_notify_log(event_id);
        }
    }
}

async fn publish_notification(feed: &Channel, client: &Client) -> Result<EventId, Box<dyn Error>> {
    let episode = Episode { title: feed.items[0].title().unwrap().to_string().clone() };
    let event_text = episode.to_notification(feed.title.clone());
    let event_id = client.publish_text_note(event_text, &[]).await?;
    Ok(event_id)
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
