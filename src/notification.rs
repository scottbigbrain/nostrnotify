use rss::{ Item, Channel, extension::* };

#[derive(PartialEq, Debug)]
pub struct StrippedChannel {
    pub title: String,
    pub episodes: Vec<Episode>,
    pub live_items: Vec<LiveItem>,
}

impl StrippedChannel {
    pub fn from_channel(channel: &Channel) -> StrippedChannel {
        let episodes: Vec<Episode> = channel.items().iter().map(|x| Episode::from_item(x)).collect();

        let live_item_extensions = get_live_item_extensions(channel.extensions()).unwrap();
        let live_items: Vec<LiveItem> = live_item_extensions.iter().map(|x| LiveItem::from_extension(x)).collect();
        
        StrippedChannel { title: String::from(channel.title()), episodes: episodes, live_items: live_items }
    }
}

fn get_live_item_extensions(extension_map: &ExtensionMap) -> Option<Vec<Extension>> {
    if !extension_map.contains_key("podcast") { return None; }
    
    let extensions = extension_map.get_key_value("podcast").unwrap().1;
    if !extensions.contains_key("liveItem") { return None; }
    
    let live_item_extensions = extensions.get_key_value("liveItem").unwrap().1.clone();
    Some(live_item_extensions)
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

impl LiveItem {
    fn from_extension(extension: &Extension) -> Self {
        let status = match extension.attrs().get("status").unwrap().as_str() {
            "pending" => LiveItemStatus::Pending,
            "live" => LiveItemStatus::Live,
            "ended" => LiveItemStatus::Ended,
            _ => LiveItemStatus::Ended,
        };
        LiveItem { status: status, start_time: extension.attrs().get("start").unwrap().clone() }
    }
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
