use std::{
    self,
    error::Error,
    time::{ Duration, SystemTime },
};
use time::OffsetDateTime;
use nostr_sdk::prelude::*;
use rss::Channel;
use reqwest;
use tokio;
use confy;
use clap::Parser;
use crate::config::*;
use crate::cli::*;

pub mod config;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut cfg: Config = confy::load("nostrnotify", None)?;
    
    match &cli.command {
        Commands::Run => monitor_mode(cfg).await?,
        Commands::SecretKey { secret_key } => {
            cfg.secret_key = String::from(secret_key);
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
            print_update_log(event_id);
        }
    }
}

fn print_check_log(feed_url: &String) {
    let check_timestamp: OffsetDateTime = SystemTime::now().into();
    println!("Requested {feed_url} at {check_timestamp}");
}

fn print_update_log(event_id: EventId) {
    let notifcation_timestamp: OffsetDateTime = SystemTime::now().into();
    println!("Broadcasted new episode notifcation at {notifcation_timestamp} with {event_id}")
}

async fn publish_notification(feed: &Channel, client: &Client) -> Result<EventId, Box<dyn Error>> {
    let event_text = format!(
        "New {pod_title} episode out now!\n\"{ep_title}\"", 
        pod_title = feed.title,
        ep_title = feed.items[0].title().unwrap(),);
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
