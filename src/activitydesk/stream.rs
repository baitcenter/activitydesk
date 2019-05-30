use crate::activitydesk::account::Identity;
use crate::indieweb::microsub as Microsub;
use crate::mastodon::stream as Mastodon;

pub trait Post {
    fn body(&self) -> String;
    fn displayed_name(&self) -> String;
    fn url(&self) -> String;
    fn avatar_image_url(&self) -> String;
}

pub trait Sink {
    fn name(&self) -> String;
    fn start(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn kind(&self) -> String;
    fn identity(&self) -> Identity;
    fn get_post_by_index(&self, index: i32) -> Option<Box<Post>>;
    fn posts(&self) -> Vec<Box<Post>>;
}

pub trait Builder {
    fn build(identity: &Identity, kind: &str) -> Option<Box<Sink>>;
    fn build_all(identity: &Identity) -> Vec<Option<Box<Sink>>>;
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
        _ => None
    }
}
