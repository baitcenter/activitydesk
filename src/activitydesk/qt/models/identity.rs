use crate::activitydesk::account::Identity;
use crate::activitydesk::settings;
use qmetaobject::*;
use std::collections::HashMap;

const ITEM_ROLE_NAME: i32 = USER_ROLE;
const ITEM_ROLE_IMAGE: i32 = USER_ROLE + 1;
const ITEM_ROLE_URL: i32 = USER_ROLE + 2;

#[derive(Default, Clone, Debug, QGadget)]
pub struct Item {
    pub handle: Identity,
}

#[derive(Default, QObject)]
pub struct Model {
    base: qt_base_class!(trait QObject),
    identity: Item,
    profile_image_url: qt_property!(QString; READ get_profile_image_url NOTIFY updated),
    url: qt_property!(QString; READ get_url NOTIFY updated),
    ident: qt_property!(Item; READ get_identity NOTIFY updated),
    load: qt_method!(fn(&mut self, identity_url: String) -> bool),
    updated: qt_signal!(),
}

impl Model {
    pub fn get_url(&self) -> QString {
        self.identity.handle.user.url.clone().into()
    }
    pub fn get_identity(&self) -> Item {
        self.identity.clone()
    }
    pub fn get_profile_image_url(&self) -> QString {
        QString::from(self.identity.handle.user.image_url.clone())
    }

    pub fn load(&mut self, identity_url: String) -> bool {
        match settings::list_all_secure() {
            Some(identities) => {
                let ident = identities
                    .iter()
                    .find(|identity| identity.user.url == identity_url)
                    .cloned();

                if ident.is_some() {
                    self.identity.handle = ident.unwrap();
                    self.updated();
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }
}

#[derive(Default, QObject)]
pub struct List {
    base: qt_base_class!(trait QAbstractListModel),
    pub identities: Vec<Item>,
    add_all_from_system: qt_method!(fn(&mut self)),
    get_identity_url: qt_method!(fn(&self, index: i32) -> QString),
    updated: qt_signal!(),
}

impl QAbstractListModel for List {
    fn row_count(&self) -> i32 {
        self.identities.len() as i32
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(ITEM_ROLE_NAME.into(), "name".into());
        map.insert(ITEM_ROLE_IMAGE.into(), "image".into());
        map.insert(ITEM_ROLE_URL.into(), "url".into());
        map
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let item_str = match self.identities.get(index.row() as usize) {
            Some(item) => match role {
                ITEM_ROLE_NAME => item.handle.user.username.clone(),
                ITEM_ROLE_IMAGE => item.handle.user.image_url.clone(),
                ITEM_ROLE_URL => item.handle.user.url.clone(),
                _ => String::default(),
            },
            _ => String::default(),
        };

        QString::from(item_str).into()
    }
}

impl List {
    pub fn get_identity_url(&self, index: i32) -> QString {
        let item_result = self.identities.get(index as usize);
        return QString::from(match item_result {
            Some(item) => item.handle.user.url.clone(),
            _ => String::default(),
        });
    }

    pub fn add_all_from_system(&mut self) {
        self.identities = settings::list_all_secure()
            .expect("Failed to fetch all accounts.")
            .iter()
            .map(|identity| {
                let mut item = Item::default();
                item.handle = identity.clone();
                item
            })
            .collect();
        let end = self.identities.len() as i32;
        QAbstractListModel::begin_insert_rows(self, 0, end - 1);
        QAbstractListModel::end_insert_rows(self);
    }
}
