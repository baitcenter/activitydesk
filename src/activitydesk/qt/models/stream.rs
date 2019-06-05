use crate::activitydesk::account;
use crate::activitydesk::stream;
use qmetaobject::*;
use std::collections::HashMap;

const ITEM_ROLE_DISPLAYED_NAME: i32 = USER_ROLE;
const ITEM_ROLE_AVATAR_IMAGE_URL: i32 = USER_ROLE + 1;
const ITEM_ROLE_PROFILE_URL: i32 = USER_ROLE + 2;
const ITEM_ROLE_CONTENT: i32 = USER_ROLE + 3;
const ITEM_ROLE_URL: i32 = USER_ROLE + 4;

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
    polling_thread: Option<std::thread::JoinHandle<()>>,
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
        map.insert(ITEM_ROLE_CONTENT.into(), "content".into());
        map.insert(ITEM_ROLE_URL.into(), "url".into());
        println!("{:#?}", map.clone());
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
                sink.start();
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
        let count = match &self.sink {
            Some(sink) => sink.posts().len() as i32,
            _ => 0,
        };
        println!("Computed {:#?} available posts.", count);
        count
    }

    fn get_field_for_post(&self, index: i32, role: i32) -> Option<QVariant> {
        match &self.sink {
            Some(sink) => {
                let post = sink.get_post_by_index(index)?.clone();
                let value = match role {
                    ITEM_ROLE_DISPLAYED_NAME => post.clone().displayed_name,
                    ITEM_ROLE_AVATAR_IMAGE_URL => post.clone().avatar_image_url,
                    ITEM_ROLE_PROFILE_URL => post.clone().identity.user.url,
                    ITEM_ROLE_CONTENT => post.clone().body,
                    ITEM_ROLE_URL => post.clone().url,
                    _ => String::default(),
                };
                println!("Showing {:#?} for {:#?}", value.clone(), post.clone());
                Some(QString::from(value).into())
            }
            _ => None,
        }
    }

    fn inflate(&mut self, posts: Vec<stream::Post>) {
        let count = posts.len() as i32;
        QAbstractListModel::begin_reset_model(self);
        QAbstractListModel::begin_insert_rows(self, 0, count);
        QAbstractListModel::end_insert_rows(self);
        QAbstractListModel::end_reset_model(self);
        println!("{:#} posts rendered.", count);
    }

    fn begin_polling(&mut self) {
        let posts = match &self.sink {
            Some(sink) => sink.posts(),
            _ => vec![],
        };
        self.inflate(posts);

        let qptr = QPointer::from(&*self);
        let cb = qmetaobject::queued_callback(move |_: ()| {
            qptr.as_pinned().map(|d_ptr| {
                let mut d = d_ptr.borrow_mut();
                d.begin_polling();
            });
        });

        let title = self.title().clone();
        self.polling_thread = Some(std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::new(15, 0));
            println!("Checking for more items in 3 seconds for {:#?}...", title);
            cb(());
        }));
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

        QAbstractListModel::begin_reset_model(self);
        self.begin_polling();
        QAbstractListModel::end_reset_model(self);

        self.sink.is_some()
    }
}
