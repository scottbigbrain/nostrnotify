use rss::{ Item, Channel };

#[derive(PartialEq, Debug)]
pub struct StrippedChannel {
    pub title: String,
    pub episodes: Vec<Episode>,
    pub live_items: Vec<LiveItem>,
}

impl StrippedChannel {
    pub fn from_channel(channel: &Channel) -> StrippedChannel {
        let episodes: Vec<Episode> = channel.items().iter().map(|x| Episode::from_item(x)).collect();
        
        StrippedChannel { title: String::from(channel.title()), episodes: episodes, live_items: vec![] }
    }
}

pub trait ToNotification {
    fn to_notification(&self, podcast_name: String) -> String;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Episode {
    pub title: String,
}

impl Episode {
    pub fn from_item(item: &Item) -> Episode {
        Episode { title: String::from(item.title().unwrap()) }
    }
}

impl ToNotification for Episode {
    fn to_notification(&self, podcast_name: String) -> String {
       format!("New episode: {} uploaded {}", podcast_name, self.title)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LiveItem {
    pub status: LiveItemStatus,
    pub start_time: String,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LiveItemStatus {
    Pending,
    Live,
    Ended
}

impl ToNotification for LiveItem {
    fn to_notification(&self, podcast_name: String) -> String {
        match self.status {
            LiveItemStatus::Pending => pending_notification(&podcast_name, &self.start_time),
            LiveItemStatus::Live => live_notification(&podcast_name),
            LiveItemStatus::Ended => ended_notification(&podcast_name),
        }
    }
}

fn pending_notification(name: &String, start_time: &String) -> String {
    format!("Live stream: {name} will be live at {start_time}")
}

fn live_notification(name: &String) -> String {
    format!("Live stream: {name} is now live!")
}

fn ended_notification(name: &String) -> String {
    format!("Live stream: {name} stopped streaming")
}

#[derive(Clone)]
pub enum NewContent {
    NewEpisode(Episode),
    NewLiveItem(LiveItem),
}
