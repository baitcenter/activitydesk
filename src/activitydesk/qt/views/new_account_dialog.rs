use crate::activitydesk::account::AuthenticableAccount;
use crate::activitydesk::account::{build_for, resolve_service_type};
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    handle: Box<AuthenticableAccount>,

    prepare_for: qt_method!(fn prepare_for(&mut self, account_type: String, profile_url: String) {
        self.handle = build_for(account_type.as_str(), profile_url.as_str());
    }),

    resolve_service_type: qt_method!(fn resolve_service_type(&mut self, url: String) -> QString {
        return match resolve_service_type(url.as_str()) {
            Some(result) => QString::from(result),
            _ => QString::from("unknown")
        };
    }),

    get_url: qt_method!(fn get_url(&mut self) -> QString {
        return match self.handle.get_authorization_url() {
            Some(url) => QString::from(url),
            _ => QString::from("")
        }
    }),

    get_token: qt_method!(fn get_token(&mut self, authorization_code: String) -> QString {
        return match self.handle.get_authentication_token(authorization_code.as_str()) {
            Some(token) => QString::from(token),
            _ => QString::from("")
        }
    }),
}
