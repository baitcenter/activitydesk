use crate::activitydesk::account::{network_for, Authenticator, Identity, User};
use qmetaobject::*;

struct AccountHandle {
    handle: Option<Box<Authenticator>>,
}

impl AccountHandle {
    fn new() -> Self {
        Self { handle: None }
    }
}

impl Default for AccountHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(QObject, Default)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    account: AccountHandle,
    user: User,
    pub account_token: qt_property!(QString),
    user_url: qt_property!(QString; READ user_url),
    user_image_url: qt_property!(QString; READ user_image_url),

    prepare_account_for: qt_method!(fn(&mut self, profile_url: String)),
    get_url: qt_method!(fn(&self) -> QString),
    has_token: qt_method!(fn(&self) -> bool),
    can_login: qt_method!(fn(&self) -> bool),
    obtain_token: qt_method!(fn(&mut self, code: String)),
    resolve_user: qt_method!(fn(&mut self) -> bool),
    result: qt_method!(fn(&self) -> QString),
}

impl Handler {
    pub fn user_url(&self) -> QString {
        return match self.account.handle.as_ref() {
            Some(_) => QString::from(self.user.url.clone()),
            _ => QString::from(""),
        };
    }

    pub fn user_image_url(&self) -> QString {
        return match self.account.handle.as_ref() {
            Some(_) => QString::from(self.user.image_url.clone()),
            _ => QString::from(""),
        };
    }

    pub fn resolve_user(&mut self) -> bool {
        return match self.account.handle.as_ref() {
            Some(handle) => {
                self.user = handle.resolve_user();
                return true;
            }
            _ => false,
        };
    }

    pub fn obtain_token(&mut self, code: String) {
        println!("Attempting to use code {:?}", code);
        match self.account.handle.as_mut() {
            Some(handle) => match handle.obtain_access(code.as_str()) {
                Some(token) => {
                    self.account_token = QString::from(token);
                    println!("Obtained token: {:?}", self.account_token);
                }
                None => println!("Code not valid."),
            },
            _ => (),
        };
    }

    pub fn result(&self) -> QString {
        let result = match self.account.handle.as_ref() {
            Some(handle) => Identity {
                user: self.user.clone(),
                access_data: handle
                    .generate_access_info()
                    .expect("Failed to serialize information for accessing this account."),
                network_type: handle.network_type(),
            },
            _ => Identity::default(),
        };

        return QString::from(serde_json::to_string(&result).unwrap());
    }

    pub fn prepare_account_for(&mut self, profile_url: String) {
        self.account.handle = network_for(profile_url.as_str());
    }

    pub fn can_login(&self) -> bool {
        return self.account.handle.is_some();
    }

    pub fn get_url(&self) -> QString {
        return match self.account.handle.as_ref() {
            Some(handle) => QString::from(handle.get_authentication_url().unwrap()),
            _ => QString::from(""),
        };
    }

    pub fn has_token(&self) -> bool {
        self.account_token.to_string().is_empty() == false
    }
}
