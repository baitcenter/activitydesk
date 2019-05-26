use crate::activitydesk::account::Identity;
use crate::activitydesk::settings;
use qmetaobject::*;
use std::collections::HashMap;

const ITEM_ROLE_Name: i32 = USER_ROLE;
const ITEM_ROLE_Image: i32 = USER_ROLE + 1;
const ITEM_ROLE_Url: i32 = USER_ROLE + 2;

#[derive(Default, QGadget)]
pub struct Item {
    pub handle: Identity,
}

#[derive(Default, QObject)]
pub struct List {
    base: qt_base_class!(trait QAbstractListModel),
    pub identities: Vec<Item>,
    add_all_from_system: qt_method!(fn(&mut self)),
}

impl QAbstractListModel for List {
    fn row_count(&self) -> i32 {
        self.identities.len() as i32
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(ITEM_ROLE_Name.into(), "name".into());
        map.insert(ITEM_ROLE_Image.into(), "image".into());
        map.insert(ITEM_ROLE_Url.into(), "url".into());
        map
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let item = self
            .identities
            .get(index.row() as usize)
            .expect("Invalid index for item.");

        let item = match role {
            ITEM_ROLE_Name => item.handle.user.username.clone(),
            ITEM_ROLE_Image => item.handle.user.image_url.clone(),
            ITEM_ROLE_Url => item.handle.user.url.clone(),
            _ => String::default(),
        };

        QString::from(item).into()
    }
}

impl List {
    pub fn add_all_from_system(&mut self) {
        self.identities = settings::list_all_secure()
            .expect("Failed to fetch all accounts.")
            .iter()
            .map(|identity| Item {
                handle: identity.clone(),
            })
            .collect();
        let end = self.identities.len() as i32;
        (self as &mut QAbstractListModel).begin_insert_rows(0, end - 1);
        (self as &mut QAbstractListModel).end_insert_rows();
    }
}
