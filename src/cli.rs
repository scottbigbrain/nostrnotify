use std::{ time::SystemTime, error::Error };
use time::OffsetDateTime;
use nostr_sdk::EventId;
use clap::{ Parser, Subcommand };
use dialoguer::{ Confirm, Input, Editor };
use confy;
use nostr_sdk::prelude::*;
use crate::config::Config;
use crate::notification::NewContent;

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

pub fn print_notify_log(new_content: NewContent, event_id: EventId) {
    let notifcation_timestamp: OffsetDateTime = SystemTime::now().into();
    let notification_type = match new_content {
        NewContent::NewEpisode(..) => "new episode",
        NewContent::NewLiveItem(..) => "live item",
    };
    println!("Broadcasted {notification_type} notifcation at {notifcation_timestamp} with {event_id}")
}

pub fn generate_keys(mut cfg: Config) -> Result<(), Box<dyn Error>> {
    let keys = Keys::generate();
    let public_key = keys.public_key().to_bech32().unwrap();
    let secret_key = keys.secret_key().unwrap().to_bech32().unwrap();
    println!("Keys Generated\nPublic Key: {public_key}\nPrivate Key: {secret_key}");
    
    let store_keys = Confirm::new()
        .with_prompt("Do you want to store the new keys to config? This will overwrite the currently stored key.")
        .interact()?;
    
    if store_keys {
        cfg.secret_key = secret_key;
        cfg.public_key = public_key;
        confy::store("nostrnotify", None, cfg)?;
        println!("Keys stored to {}", confy::get_configuration_file_path("nostrnotify", None).unwrap().to_str().unwrap());
    } else {
        println!("Gotcha. Keys not stored");
    }
    
    Ok(())
}

pub fn request_user_metadata(mut cfg: Config) -> Result<(), Box<dyn Error>> {
    let username: String = Input::new()
        .with_prompt("Enter bot username")
        .interact_text()?;
    cfg.name = username;
    let display_name: String = Input::new()
        .with_prompt("Enter bot display name")
        .interact_text()?;
    cfg.display_name = display_name;
        
    let include_description = Confirm::new()
        .with_prompt("Do you want to include a description?")
        .interact()?;
    
    let description = if include_description {
        Editor::new().edit("Enter bot description").unwrap().unwrap()
    } else {
        String::new()
    };
    cfg.description = description;
    
    confy::store("nostrnotify", None, cfg)?;
    Ok(())
}
