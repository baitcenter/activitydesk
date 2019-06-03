use crate::activitydesk::account::Identity;
use crate::activitydesk::stream;

pub struct Sink {
    pub identity: Identity,
    pub channel: String,
}

impl stream::Sink for Sink {
    fn add_receiver(&mut self, _cb: Box<stream::ReceiveCallback>) {}
    fn identity(&self) -> Identity {
        self.identity.clone()
    }
    fn kind(&self) -> String {
        self.channel.clone()
    }
    fn name(&self) -> String {
        self.channel.clone()
    }
    fn start(&mut self) -> bool {
        false
    }
    fn stop(&mut self) -> bool {
        false
    }
    fn get_post_by_index(&self, _index: i32) -> Option<Box<stream::Post>> {
        None
    }
    fn posts(&self) -> Vec<Box<stream::Post>> {
        vec![]
    }
}

impl stream::Builder for Sink {
    fn build_all(identity: &Identity) -> Vec<Option<Box<stream::Sink>>> {
        vec![Self::build(identity, "notifications")]
    }

    fn build(identity: &Identity, kind: &str) -> Option<Box<stream::Sink>> {
        Some(Box::new(Sink {
            identity: identity.clone(),
            channel: kind.into(),
        }))
    }
}
