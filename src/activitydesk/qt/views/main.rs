use crate::activitydesk::account::Identity;
use crate::activitydesk::settings;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    register_new_account: qt_method!(fn(&mut self, result: String) -> ()),
    load_streams: qt_method!(fn(&mut self) -> ()),
}

impl Handler {
    pub fn load_streams(&mut self) {
        for identity in settings::list_all_secure() {}
    }

    pub fn register_new_account(&mut self, identity_str: String) {
        match Identity::from_string(identity_str.as_str()) {
            Some(identity) => {
                // TODO: Save account information to system.
                println!("Obtained a usable identity: {:?}", identity);
                identity.store();
                // TODO: Inform system of new account.
            }
            _ => {}
        }
    }
}
