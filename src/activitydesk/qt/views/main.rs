use super::new_account_dialog::Result as NewAccountDialogResult;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    register_new_account: qt_method!(fn(&mut self, result: String) -> ()),
}

impl Handler {
    pub fn register_new_account(&mut self, result: String) {
        match serde_json::from_str::<NewAccountDialogResult>(result.as_str()) {
            Ok(_result) => {
                // TODO: Save account information to system.
                // TODO: Inform system of new account.
            }
            _ => {}
        }
    }
}
