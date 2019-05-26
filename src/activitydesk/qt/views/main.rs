use crate::activitydesk::account::Identity;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    register_new_account: qt_method!(fn(&mut self, result: String) -> ()),
}

impl Handler {
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
