use crate::activitydesk::account::Identity;
use crate::activitydesk::stream;
use elefren::http_send::HttpSender;
use elefren::Data;
use elefren::Mastodon;
use elefren::MastodonClient;
use elefren::entities::event::Event;
use elefren::entities::status::Status;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use futures::*;

#[derive(Debug, Default, Clone)]
pub struct Post {
    status: Option<Status>
}

pub struct Sink {
    pub identity: Identity,
    pub kind: String,
    thread: Option<thread::JoinHandle<()>>,
    post_receiver: Option<mpsc::Receiver<Post>>,
}

impl stream::Post for Post {
    fn body(&self) -> String { let status = &(self as &Post).status; match status { Some(masto_status) => masto_status.content.clone(), _ => String::default() } }
    fn displayed_name(&self) -> String { let status = &(self as &Post).status; match status { Some(masto_status) => masto_status.account.display_name.clone(), _ => String::default() } }
    fn url(&self) -> String { let status = &(self as &Post).status; match status { Some(masto_status) => masto_status.url.clone().unwrap_or(masto_status.uri.clone()), _ => String::default() } }
    fn avatar_image_url(&self) -> String { let status = &(self as &Post).status; match status { Some(masto_status) => masto_status.account.avatar.clone(), _ => String::default() } }
}

impl Sink {
    fn get_app(&self) -> Mastodon<HttpSender> {
        let masto_data: Data = serde_json::from_str(self.identity.access_data.as_str())
            .ok()
            .expect("Failed to deserialize Mastodon authentication information.");
        Mastodon::from(masto_data)
    }
}

impl futures::Stream for Sink {
    type Item = Post;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Post>, ()> {
        Err(())
    }
}

impl stream::Sink for Sink {
    fn identity(&self) -> Identity {
        self.identity.clone()
    }
    fn kind(&self) -> String {
        self.kind.clone()
    }
    fn name(&self) -> String {
        self.kind.clone()
    }
    fn start(&mut self) -> bool {
        let builder = std::thread::Builder::new();
        let app = (self as &Sink).get_app();

        let thread = builder.spawn(move || {
            match app.streaming_user() {
                Ok(stream) => {
                    for event in stream {
                        match event {
                            Event::Update(status) => {
                                let post = Post { status: Some(status) };
                            },
                            _ => {}
                        }
                    }
                },
                _ => {},
            }
        });

        println!("Did thread start? {:#?}", thread);
        self.thread = thread.ok();
        self.thread.is_some()
    }

    fn stop(&mut self) -> bool {
        // TODO: Stop background thread for stream.
        false
    }
    fn get_post_by_index(&self, index: i32) -> Option<Box<stream::Post>> { None }
    fn posts(&self) -> Vec<Box<stream::Post>> { vec![] }
}

impl stream::Builder for Sink {
    fn build_all(identity: &Identity) -> Vec<Option<Box<stream::Sink>>> {
        vec![
            Self::build(identity, "timeline:home"),
            Self::build(identity, "timeline:public"),
        ]
    }

    fn build(identity: &Identity, kind: &str) -> Option<Box<stream::Sink>> {
        Some(Box::new(Sink {
            identity: identity.clone(),
            kind: kind.into(),
            thread: None,
            post_receiver: None
        }))
    }
}
