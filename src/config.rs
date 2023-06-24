use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub secret_key: String,
    pub public_key: String,
    pub relays: Vec<String>,
    pub feeds: Vec<String>,
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
            feeds: vec![],
            check_interval_seconds: 300,
            name: "".into(),
            display_name: "".into(),
            description: "".into(),
        }
    }
}
