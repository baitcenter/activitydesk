use crate::activitydesk::account::Identity;
use crate::activitydesk::stream;
use elefren::entities::event::Event;
use elefren::entities::status::Status;
use elefren::errors::Error;
use elefren::http_send::HttpSender;
use elefren::prelude::*;
use futures::future::lazy;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[derive(Clone)]
struct Streamer {
    identity: Identity,
    kind: String,
    sender: RefCell<stream::ChannelSender>,
}

pub struct Sink {
    pub identity: Identity,
    pub kind: String,
    runner: Option<Runtime>,
    streamer: Option<RefCell<Streamer>>,
    receiver: Option<Arc<Mutex<stream::ChannelReceiver>>>,
    recv_thread: Option<std::thread::JoinHandle<()>>,
    backlog: Arc<Mutex<Vec<stream::Post>>>,
}

impl Streamer {
    fn app(&self) -> Mastodon<HttpSender> {
        let masto_data: Data = serde_json::from_str(&self.identity.access_data.clone())
            .ok()
            .expect("Failed to deserialize Mastodon authentication information.");
        Mastodon::from(masto_data)
    }

    pub fn send(&self, post: Option<stream::Post>) {
        match self.sender.borrow_mut().send(post.clone()) {
            Err(err) => eprintln!("Failed to send post upstream: {:#?}", err),
            _ => {}
        }
    }

    pub fn get(&self) -> elefren::errors::Result<<Mastodon as MastodonClient>::Stream> {
        let app = self.app();
        println!("Fetching streaming client..");

        let result = match self.kind.as_str() {
            "timeline:home" => app.streaming_user(),
            "timeline:public" => app.streaming_public(),
            "timeline:local" => app.streaming_local(),
            kind => {
                if kind.starts_with("hashtag:public:") {
                    let (hashtag, _) = kind.split_at("hashtag:public:".len());
                    app.streaming_public_hashtag(hashtag)
                } else if kind.starts_with("hashtag:local:") {
                    let (hashtag, _) = kind.split_at("hashtag:local:".len());
                    app.streaming_local_hashtag(hashtag)
                } else if kind.starts_with("list:") {
                    let (list_id, _) = kind.split_at("list:".len());
                    app.streaming_list(list_id)
                } else {
                    Err(Error::Other("Invalid kind of stream for Mastodon".into()))
                }
            }
        };

        println!("Fetched a new stream: {:#?}", result);
        result
    }

    pub fn populate(&self) {
        let app = self.app();
        println!("Backfilling feed...");
        let statuses = match self.kind.as_str() {
            "timeline:home" => match app.get_home_timeline() {
                Err(_) => vec![],
                Ok(page) => page.items_iter().collect(),
            },
            "timeline:public" => match app.get_public_timeline(false) {
                Err(_) => vec![],
                Ok(items) => items,
            },
            "timeline:local" => match app.get_public_timeline(true) {
                Err(_) => vec![],
                Ok(items) => items,
            },
            _ => vec![],
        };

        println!("Backfilling {:#} items.", statuses.len());

        statuses.iter().for_each(|status| {
            self.send(Some(build_post(&self.identity, status)));
        });
    }
}

impl Sink {
    fn build_stream(&mut self) -> Option<Runtime> {
        let (sender, receiver): (stream::ChannelSender, stream::ChannelReceiver) =
            stream::get_transfer();
        let identity = self.identity.clone();
        let mut runner = Runtime::new().ok()?;

        self.receiver = Some(Arc::new(Mutex::new(receiver)));
        self.streamer = Some(RefCell::new(Streamer {
            identity: identity.clone(),
            kind: self.kind.clone(),
            sender: RefCell::new(sender.clone()),
        }));

        let streamer_backfill = self.streamer.as_ref().unwrap().clone();
        runner.spawn(lazy(move || {
            streamer_backfill.borrow().populate();
            Ok(())
        }));

        let streamer_stream = self.streamer.as_ref().unwrap().clone();
        runner.spawn(lazy(move || {
            match streamer_stream.borrow().get() {
                Ok(stream) => {
                    for event in stream {
                        let result = match event {
                            Event::Update(status) => Some(build_post(&identity, &status)),
                            Event::Notification(_notif) => None,
                            _ => None,
                        };
                        streamer_stream.borrow().send(result);
                    }
                }
                _ => {}
            };
            println!("Stopped streaming.");
            Ok(())
        }));

        println!(
            "Built thread of listening to Mastodon feed {:#?}",
            self.kind
        );
        Some(runner)
    }

    fn begin_receiving(&mut self) {
        println!("Beginning to listen to responses from receiver.");
        match &self.receiver {
            None => {
                println!("No receiver to listen on.");
            }
            Some(rx) => {
                let mutex = rx.clone();
                let backlog_mutex = self.backlog.clone();

                let builder = std::thread::Builder::new()
                    .name(format!("{:#?}:{:#?}", self.identity.network_type, self.kind).into());

                self.recv_thread = builder
                    .spawn(move || {
                        mutex.lock().unwrap().deref_mut().iter().for_each(
                            move |post: Option<stream::Post>| {
                                match backlog_mutex.lock() {
                                    Ok(mut mtx) => {
                                        if post.is_some() {
                                            mtx.push(post.clone().unwrap());
                                        }
                                    }
                                    Err(_) => {}
                                };
                            },
                        );
                    })
                    .ok();
            }
        }
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
        self.runner = (self as &mut Sink).build_stream();
        (self as &mut Sink).begin_receiving();

        self.runner.is_some()
    }

    fn stop(&mut self) -> bool {
        false
    }

    fn get_post_by_index(&self, index: i32) -> Option<stream::Post> {
        self.backlog
            .try_lock()
            .ok()?
            .deref()
            .clone()
            .get(index as usize)
            .map(|post| post.clone())
    }

    fn posts(&self) -> Vec<stream::Post> {
        println!("Getting all of the post for {:#?}", self.kind());

        match self.backlog.try_lock() {
            Ok(posts) => {
                println!("Got {:#?} posts for {:#?}", posts.len(), self.kind());
                posts.deref().clone()
            }
            Err(err) => {
                println!("Couldn't lock backlog: {:#?}", err);
                vec![]
            }
        }
    }
}

impl stream::Builder for Sink {
    fn build_all(identity: &Identity) -> Vec<Option<Box<stream::Sink>>> {
        ["timeline:home", "timeline:public"]
            .iter()
            .map(|feed| Self::build(identity, feed))
            .collect()
    }

    fn build(identity: &Identity, kind: &str) -> Option<Box<stream::Sink>> {
        Some(Box::new(Sink {
            identity: identity.clone(),
            kind: kind.into(),
            recv_thread: None,
            backlog: Arc::new(Mutex::new(vec![])),
            runner: None,
            streamer: None,
            receiver: None,
        }))
    }
}

fn build_post(identity: &Identity, status: &Status) -> stream::Post {
    stream::Post {
        body: status.content.clone(),
        displayed_name: status.account.display_name.clone(),
        url: status.url.clone().unwrap_or(status.uri.clone()),
        avatar_image_url: status.account.avatar.clone(),
        identity: identity.clone(),
    }
}
