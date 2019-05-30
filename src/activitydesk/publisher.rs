use crate::activitydesk::account::Identity;
use crate::indieweb::micropub::Composer as Micropub;
use crate::mastodon::publisher::Composer as Mastodon;

#[derive(Debug, PartialEq)]
pub enum Status {
    SuccessfulSend,
    FailedToSend,
}

pub struct Media {
    pub content_type: String, // mime-type?
    pub url: String,
    pub alt_text: String,
    pub identifier: String,
}

// Due to the need to keep cross-network compat, this'll mirror ActivityStreams properties
// more than the other platform at the implementation (Microformats).
pub struct Post {
    pub summary: String,
    pub content: String,
    pub sensitive: bool,
}

pub trait Publisher {
    fn send(&self, post: &Post) -> Option<Status>;
    fn upload_media(&self, media: &Media) -> Option<Status>;
}

pub trait Builder {
    fn build(identity: &Identity) -> Option<Box<Publisher>>;
}

pub fn build(identity: &Identity) -> Option<Box<Publisher>> {
    match identity.network_type.as_str() {
        "indieweb" => Micropub::build(identity),
        "mastodon" => Mastodon::build(identity),
        _ => None,
    }
}

pub fn async_send(identity: Identity, post: Post) {
    std::thread::spawn(move || {
        let publisher = build(&identity);
        if publisher.is_some() {
            let publsr = publisher.unwrap();
            publsr
                .send(&post)
                .expect("No useful result from sending post.");
        } else {
            eprintln!(
                "No publisher found for the {:?} publishing interface.",
                identity.network_type
            );
        }
    });
}
