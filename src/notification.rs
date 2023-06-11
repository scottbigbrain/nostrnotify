pub struct Notification(String);

pub trait ToNotification {
    fn to_notification(&self) -> String;
}

pub struct Episode {
    pub title: String,
    pub podcast_name: String,
}

impl ToNotification for Episode {
    fn to_notification(&self) -> String {
       format!("New episode: {} uploaded {}", self.podcast_name, self.title)
    }
}

pub struct LiveItem {
    pub status: LiveItemStatus,
    pub start_time: String,
    pub podcast_name: String,
}

#[derive(Clone, Copy)]
pub enum LiveItemStatus {
    Pending,
    Live,
    Ended
}

impl ToNotification for LiveItem {
    fn to_notification(&self) -> String {
        match self.status {
            LiveItemStatus::Pending => pending_notification(&self.podcast_name, &self.start_time),
            LiveItemStatus::Live => live_notification(&self.podcast_name),
            LiveItemStatus::Ended => ended_notification(&self.podcast_name),
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
