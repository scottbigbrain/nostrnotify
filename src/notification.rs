use rss::{ Item, Channel, extension::* };

#[derive(PartialEq, Debug)]
pub struct StrippedChannel {
    pub url: String,
    pub title: String,
    pub episodes: Vec<Episode>,
    pub live_items: Vec<LiveItem>,
}

impl StrippedChannel {
    pub fn from_channel(channel: &Channel, feed_url: &str) -> StrippedChannel {
        let episodes: Vec<Episode> = channel.items().iter().map(|x| Episode::from_item(x)).collect();

        let live_item_extensions = get_live_item_extensions(channel.extensions());
        let live_items: Vec<LiveItem> = live_item_extensions.iter().map(|x| LiveItem::from_extension(x)).collect();
        
        StrippedChannel { url: String::from(feed_url), title: String::from(channel.title()), episodes: episodes, live_items: live_items }
    }
}

fn get_live_item_extensions(extension_map: &ExtensionMap) -> Vec<Extension> {
    if !extension_map.contains_key("podcast") { return vec![]; }
    
    let extensions = extension_map.get_key_value("podcast").unwrap().1;
    if !extensions.contains_key("liveItem") { return vec![]; }
    
    extensions.get_key_value("liveItem").unwrap().1.clone()
}

pub trait ToNotification {
    fn to_notification(&self, podcast_name: String) -> String;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Episode {
    pub title: String,
    pub link: Option<String>,
}

impl Episode {
    pub fn from_item(item: &Item) -> Episode {
        Episode { title: String::from(item.title().unwrap()), link: item.link.clone() }
    }
}

impl ToNotification for Episode {
    fn to_notification(&self, podcast_name: String) -> String {
        if self.link != None {
            format!("New episode: {} uploaded '{}'\nWatch at {}", podcast_name, self.title, self.link.as_ref().unwrap())
        } else {
            format!("New episode: {} uploaded '{}'", podcast_name, self.title)
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LiveItem {
    pub status: LiveItemStatus,
    pub start_time: String,
    pub link: String,
}

impl LiveItem {
    fn from_extension(extension: &Extension) -> Self {
        let status = match extension.attrs().get("status").unwrap().as_str() {
            "pending" => LiveItemStatus::Pending,
            "live" => LiveItemStatus::Live,
            "ended" => LiveItemStatus::Ended,
            _ => LiveItemStatus::Ended,
        };
        let link = extension.children().get("contentLink").unwrap()[0].attrs().get("href").unwrap().clone();
        LiveItem { status: status, start_time: extension.attrs().get("start").unwrap().clone(), link: link }
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
            LiveItemStatus::Pending => pending_notification(&podcast_name, &self.start_time, &self.link),
            LiveItemStatus::Live => live_notification(&podcast_name, &self.link),
            LiveItemStatus::Ended => ended_notification(&podcast_name),
        }
    }
}

fn pending_notification(name: &String, start_time: &String, link: &String) -> String {
    format!("Live stream: {name} will be live at {start_time}\nWatch at {link}")
}

fn live_notification(name: &String, link: &String) -> String {
    format!("Live stream: {name} is now live!\nWatch at {link}")
}

fn ended_notification(name: &String) -> String {
    format!("Live stream: {name} stopped streaming")
}

#[derive(Clone, Debug)]
pub enum NewContent {
    NewEpisode(Episode),
    NewLiveItem(LiveItem),
}
