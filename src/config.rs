use std::error::Error;
use serde::{Serialize, Deserialize};
use dialoguer::{ Confirm, Input, Editor };
use confy;
use nostr_sdk::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub secret_key: String,
    pub public_key: String,
    pub relays: Vec<String>,
    pub feed_url: String,
    pub check_interval_seconds: u64,
    pub name: String,
    pub display_name: String,
    pub description: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            secret_key: "".into(),
            public_key: "".into(), 
            relays: vec!["wss://nos.lol".into(), "wss://relay.house".into()],
            feed_url: "".into(),
            check_interval_seconds: 300,
            name: "".into(),
            display_name: "".into(),
            description: "".into(),
        }
    }
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
