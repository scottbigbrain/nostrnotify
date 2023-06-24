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
        Commands::AddFeed { feed_url } => {
            cfg.feeds.push(String::from(feed_url));
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
    
    let mut old_feeds: Vec<StrippedChannel> = vec![];
    for feed_url in cfg.feeds.iter() {
        let verbose_feed = get_feed(&feed_url).await?;
        old_feeds.push(StrippedChannel::from_channel(&verbose_feed, feed_url));
    }
    
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.check_interval_seconds));
    
    loop {
        interval.tick().await;
        
        let mut new_feeds: Vec<StrippedChannel> = vec![];
        for old_feed in old_feeds.iter() {
            let verbose_feed = get_feed(&old_feed.url).await?;
            let new_feed = StrippedChannel::from_channel(&verbose_feed, &old_feed.url);
            print_check_log(&old_feed.url);
            
            handle_feeds(&new_feed, old_feed, &client).await?;
            new_feeds.push(new_feed);
        }
        old_feeds = new_feeds;
    }
}

async fn handle_feeds(new_feed: &StrippedChannel, old_feed: &StrippedChannel, client: &Client) -> Result<()> {
    if new_feed != old_feed {
        let new_content = handle_update(&new_feed, &old_feed);
        handle_new_content(new_content, new_feed.title.clone(), &client).await?;
    }
    Ok(())
}

fn handle_update(new_feed: &StrippedChannel, old_feed: &StrippedChannel) -> Option<NewContent> {
    if new_feed.episodes.len() > old_feed.episodes.len() {
        return Some(NewContent::NewEpisode(new_feed.episodes[0].clone()));
    }
    
    if new_feed.live_items != old_feed.live_items {
        if new_feed.live_items.len() > old_feed.live_items.len() {
            return Some(NewContent::NewLiveItem(new_feed.live_items[0].clone()));
        }
        return find_inconsistent_live_items(new_feed, old_feed);
    }
    
    None
}

fn find_inconsistent_live_items(new_feed: &StrippedChannel, old_feed: &StrippedChannel) -> Option<NewContent> {
    for i in 0..new_feed.live_items.len() {
        if new_feed.live_items[i].status != old_feed.live_items[i].status {
            return Some(NewContent::NewLiveItem(new_feed.live_items[i].clone()));
        }
    }
    None
}

async fn handle_new_content(new_content: Option<NewContent>, podcast_title: String, client: &Client) -> Result<(), Box<dyn Error>> {
    match new_content {
        Some(new_content) => {
            let event_id = publish_notification(new_content.clone(), podcast_title, client).await?;
            print_notify_log(new_content.clone(), event_id);
        },
        None => (),
    }
    Ok(())
}

async fn publish_notification(new_content: NewContent, podcast_title: String, client: &Client) -> Result<EventId, Box<dyn Error>> {
    let event_text;
    match new_content {
        NewContent::NewEpisode(episode) => {
            event_text = episode.to_notification(String::from(podcast_title));
        },
        NewContent::NewLiveItem(live_item) => {
            event_text = live_item.to_notification(String::from(podcast_title));
        }
    }
    Ok(client.publish_text_note(event_text, &[]).await?)
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
