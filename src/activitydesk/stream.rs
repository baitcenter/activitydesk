use crate::activitydesk::account::Identity;
use crate::indieweb::microsub as Microsub;
use crate::mastodon::stream as Mastodon;
use futures::sync::mpsc::{channel, Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Post {
    pub body: String,
    pub displayed_name: String,
    pub url: String,
    pub avatar_image_url: String,
    pub identity: Identity,
}

impl Into<String> for Post {
    fn into(self) -> String {
        return serde_json::to_string::<Self>(&self).unwrap_or(String::default());
    }
}

impl From<String> for Post {
    fn from(post_str: String) -> Self {
        return serde_json::from_str::<Self>(&post_str).unwrap_or(Self::default());
    }
}

pub struct ReceiveCallback {
    pub callback: Box<FnMut(Option<Post>)>,
}

impl ReceiveCallback {
    pub fn invoke(&mut self, post: Option<Post>) {
        (self.callback)(post.clone());
    }
}

pub trait Sink {
    fn name(&self) -> String;
    fn start(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn kind(&self) -> String;
    fn identity(&self) -> Identity;
    fn get_post_by_index(&self, index: i32) -> Option<Box<Post>>;
    fn posts(&self) -> Vec<Box<Post>>;
    fn add_receiver(&mut self, cb: Box<ReceiveCallback>);
}

pub trait Builder {
    fn build(identity: &Identity, kind: &str) -> Option<Box<Sink>>;
    fn build_all(identity: &Identity) -> Vec<Option<Box<Sink>>>;
}

pub type ChannelReceiver = Receiver<Option<Post>>;
pub type ChannelSender = Sender<Option<Post>>;

pub fn get_transfer() -> (ChannelSender, ChannelReceiver) {
    channel(1024)
}

pub fn get_all_sinks() -> Vec<Option<Box<Sink>>> {
    let mut sinks = vec![];
    let idents = crate::activitydesk::settings::list_all_secure();

    if idents.is_some() {
        for ident in idents.unwrap() {
            let mut sinks_for_ident = get_all(&ident);
            sinks.append(&mut sinks_for_ident);
        }
    }

    sinks
}

pub fn get_all(identity: &Identity) -> Vec<Option<Box<Sink>>> {
    match identity.network_type.as_str() {
        "mastodon" => Mastodon::Sink::build_all(identity),
        "indieweb" => Microsub::Sink::build_all(identity),
        _ => vec![],
    }
}

pub fn get(identity: &Identity, kind: &str) -> Option<Box<Sink>> {
    match identity.network_type.as_str() {
        "mastodon" => Mastodon::Sink::build(identity, kind),
        "indieweb" => Microsub::Sink::build(identity, kind),
        _ => None,
    }
}
