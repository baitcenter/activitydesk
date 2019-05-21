use crate::activitydesk::account::AuthenticableAccount;
use crate::activitydesk::profile;
use crate::mastodon::app;
use elefren::http_send::HttpSender;
use elefren::registration::{Registered, Registration};

pub fn for_user(profile_url: &str) -> Account {
    return Account::for_instance(profile_url);
}

#[derive(Default)]
pub struct Account {
    pub profile_url: Option<String>,
    pub instance_url: String,
    pub access_token: Option<String>,
    pub registration: Option<Registered<HttpSender>>,
}

impl From<Account> for profile::Profile {
    fn from(account: Account) -> Self {
        Self {
            profile_url: account.profile_url.unwrap(),
            network_type: "mastodon".into(),
        }
    }
}

impl Account {
    pub fn for_instance(instance_url: &str) -> Self {
        return match Registration::new(instance_url).register(app()) {
            Ok(reg) => Self {
                registration: None,
                profile_url: None,
                instance_url: instance_url.into(),
                access_token: None,
            },
            _ => Account::default(),
        };
    }
}

impl AuthenticableAccount for Account {
    fn get_authorization_url(&self) -> Option<String> {
        return match self.registration?.authorize_url() {
            Ok(url) => Some(url),
            _ => None,
        };
    }

    fn get_authentication_token(&self, authorization_code: &str) -> Option<String> {
        return match self.registration?.complete(authorization_code) {
            Ok(app) => Some(app.data.token.into()),
            _ => None,
        };
    }
}
