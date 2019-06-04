use crate::activitydesk::account::Identity;
use crate::activitydesk::stream;
use elefren::entities::event::Event;
use elefren::entities::status::Status;
use elefren::errors::Error;
use elefren::http_send::HttpSender;
use elefren::prelude::*;
use futures::future::lazy;
use futures::future::IntoFuture;
use futures::stream::Stream;
use std::cell::RefCell;
use tokio::runtime::current_thread::Runtime as CurrentRuntime;
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
    current_runner: Option<CurrentRuntime>,
    streamer: Option<RefCell<Streamer>>,
    receiver: Option<Box<stream::ChannelReceiver>>,
    receivers: Vec<Box<stream::ReceiveCallback>>,
}

impl Streamer {
    fn app(&self) -> Mastodon<HttpSender> {
        let masto_data: Data = serde_json::from_str(&self.identity.access_data.clone())
            .ok()
            .expect("Failed to deserialize Mastodon authentication information.");
        Mastodon::from(masto_data)
    }

    pub fn send(&self, post: Option<stream::Post>) {
        match self.sender.borrow_mut().try_send(post.clone()) {
            Err(err) => {
                eprintln!(
                    "Failed to send post {:#?} upstream: Closed? {:#?} Full? {:#?}",
                    post,
                    err.is_disconnected(),
                    err.is_full()
                );
                eprintln!("Failed to send post upstream: {:#?}", err.into_inner());
            }
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
            "timeline:direct" => app.streaming_direct(),
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
}

impl Sink {
    fn build_stream(&mut self) -> Option<Runtime> {
        let (sender, receiver): (stream::ChannelSender, stream::ChannelReceiver) =
            stream::get_transfer();
        let identity = self.identity.clone();
        let mut runner = Runtime::new().ok()?;

        self.receiver = Some(Box::new(receiver));
        self.streamer = Some(RefCell::new(Streamer {
            identity: identity.clone(),
            kind: self.kind.clone(),
            sender: RefCell::new(sender.clone()),
        }));
        let streamer = self.streamer.as_ref().unwrap().clone();

        runner.spawn(lazy(move || {
            println!("Starting listening to stream.");
            match streamer.borrow().get() {
                Ok(stream) => {
                    for event in stream {
                        let result = match event {
                            Event::Update(status) => Some(build_post(identity.clone(), status)),
                            Event::Notification(_notif) => None,
                            _ => None,
                        };
                        streamer.borrow().send(result);
                    }
                }
                _ => {}
            };

            println!("End listening to stream.");
            Ok(())
        }));

        println!(
            "Built thread of listening to Mastodon feed {:#?}",
            self.kind
        );
        Some(runner)
    }

    fn begin_receiving(&mut self) {
        match &mut self.receiver {
            Some(rx) => {
                let ft = rx
                    .as_mut()
                    .for_each(move |post: Option<stream::Post>| {
                        println!("Post: {:#?}", post);
                        Ok(())
                    })
                    .into_future();
                // self.current_runner.unwrap().spawn(ft);
                // self.current_runner.as_ref().unwrap().run();
                println!("{:#?}", self.current_runner);
            }
            _ => {}
        }
    }
}

impl stream::Sink for Sink {
    fn add_receiver(&mut self, cb: Box<stream::ReceiveCallback>) {
        self.receivers.push(cb);
    }

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
        println!("Building stream {:#?}", self.kind());
        self.current_runner = CurrentRuntime::new().ok();
        self.runner = (self as &mut Sink).build_stream();
        println!("Built stream runner for Mastodon/{:#?}", self.kind());
        (self as &mut Sink).begin_receiving();
        self.runner.is_some()
    }

    fn stop(&mut self) -> bool {
        false
    }

    fn get_post_by_index(&self, _index: i32) -> Option<Box<stream::Post>> {
        None
    }

    fn posts(&self) -> Vec<Box<stream::Post>> {
        println!("Get all of the post for {:#?}", self.kind());
        vec![]
    }
}

impl stream::Builder for Sink {
    fn build_all(identity: &Identity) -> Vec<Option<Box<stream::Sink>>> {
        [
            "timeline:home",
            "timeline:public",
            "timeline:local",
            "timeline:direct",
        ]
        .iter()
        .map(|feed| Self::build(identity, feed))
        .collect()
    }

    fn build(identity: &Identity, kind: &str) -> Option<Box<stream::Sink>> {
        Some(Box::new(Sink {
            identity: identity.clone(),
            kind: kind.into(),
            current_runner: None,
            runner: None,
            streamer: None,
            receiver: None,
            receivers: vec![],
        }))
    }
}

fn build_post(identity: Identity, status: Status) -> stream::Post {
    stream::Post {
        body: status.content.clone(),
        displayed_name: status.account.display_name.clone(),
        url: status.url.clone().unwrap_or(status.uri.clone()),
        avatar_image_url: status.account.avatar.clone(),
        identity,
    }
}
