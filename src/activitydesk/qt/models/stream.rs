use crate::activitydesk::account;
use crate::activitydesk::stream;
use qmetaobject::*;
use std::collections::HashMap;

const ITEM_ROLE_DISPLAYED_NAME: i32 = USER_ROLE;
const ITEM_ROLE_AVATAR_IMAGE_URL: i32 = USER_ROLE + 1;
const ITEM_ROLE_PROFILE_URL: i32 = USER_ROLE + 2;

#[derive(Default, Clone, Debug, QGadget)]
pub struct Model {}

#[derive(Default, QObject)]
pub struct Delegate {
    base: qt_base_class!(trait QAbstractListModel),
    removed: qt_signal!(),
    inserted: qt_signal!(),
    set_stream: qt_method!(fn(&mut self, identity_url: String, stream_kind: String) -> bool),
    start: qt_method!(fn(&mut self)),
    stop: qt_method!(fn(&mut self)),
    title: qt_method!(fn(&self) -> QString),
    sink: Option<Box<stream::Sink>>,
}

impl QAbstractListModel for Delegate {
    fn row_count(&self) -> i32 {
        self.get_count_of_posts()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(ITEM_ROLE_DISPLAYED_NAME.into(), "displayed_name".into());
        map.insert(ITEM_ROLE_AVATAR_IMAGE_URL.into(), "avatar_image_url".into());
        map.insert(ITEM_ROLE_PROFILE_URL.into(), "profile_url".into());
        map
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        match self.get_field_for_post(index.row(), role) {
            None => QString::from(String::default()).into(),
            Some(val) => val,
        }
    }
}

impl Delegate {
    fn start(&mut self) {
        match &mut self.sink {
            Some(sink) => {
                println!("Starting stream {:#?}.", sink.kind());
                if sink.start() {
                    println!("Stream {:#?} began.", sink.kind());
                    sink.add_receiver(Box::new(stream::ReceiveCallback {
                        callback: Box::new(|post: Option<stream::Post>| println!("{:#?}", post)),
                    }));
                } else {
                    eprintln!("Stream {:#?} failed.", sink.kind());
                }
            }
            _ => {}
        };
    }
    fn stop(&mut self) {
        match &mut self.sink {
            Some(sink) => {
                sink.stop();
            }
            _ => {}
        };
    }

    fn title(&self) -> QString {
        QString::from(match &self.sink {
            Some(sink) => sink.name(),
            _ => String::default(),
        })
        .into()
    }

    fn get_count_of_posts(&self) -> i32 {
        match &self.sink {
            Some(sink) => sink.posts().len() as i32,
            _ => 0,
        }
    }

    fn get_field_for_post(&self, index: i32, role: i32) -> Option<QVariant> {
        match &self.sink {
            Some(sink) => {
                let post = sink.get_post_by_index(index)?;
                Some(
                    QString::from(match role {
                        ITEM_ROLE_DISPLAYED_NAME => post.displayed_name,
                        ITEM_ROLE_AVATAR_IMAGE_URL => post.avatar_image_url,
                        ITEM_ROLE_PROFILE_URL => post.url,
                        _ => String::default(),
                    })
                    .into(),
                )
            }
            _ => None,
        }
    }

    fn set_stream(&mut self, identity_url: String, stream_kind: String) -> bool {
        match account::Identity::find_from_url(identity_url.as_str()) {
            Some(identity) => {
                self.sink = stream::get(&identity, stream_kind.as_str());
                if self.sink.is_some() {
                    self.sink.as_mut().unwrap().start();
                }
            }
            _ => {}
        }

        self.sink.is_some()
    }
}
