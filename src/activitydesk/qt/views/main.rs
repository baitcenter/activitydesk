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
        for identity in settings::list_all_secure().expect("Failed to fetch all accounts.") {
            println!(
                "Got an {:?} account for {:?}",
                identity.network_type, identity.user.url
            );
        }
    }

    pub fn register_new_account(&mut self, identity_str: String) {
        match Identity::from_string(identity_str.as_str()) {
            Some(identity) => {
                println!("Obtained a usable identity: {:?}", identity);
                identity.store();
            }
            _ => {}
        }
    }
}
