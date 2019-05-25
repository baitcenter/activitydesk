use crate::activitydesk::settings::set_secure;
use crate::indieweb::indieauth::Account as IndieAuthAccount;
use crate::mastodon::account::Account as MastodonAccount;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub url: String,
    pub service_url: String,
    pub image_url: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub user: User,
    pub network_type: String,
    pub access_data: String,
}

// Provides logic for authenticating a user to their account.
pub trait Authenticator {
    fn get_authentication_url(&self) -> Option<String>;
    fn obtain_access(&mut self, code: &str) -> Option<String>;
    fn network_type(&self) -> String;
    fn resolve_user(&self) -> User;
    fn generate_access_info(&self) -> Option<String>;
}

pub trait Builder {
    fn supported(url: &str) -> bool;
    fn build_for(url: &str) -> Option<Box<Authenticator>>;
}

impl Identity {
    pub fn from_string(identity_str: &str) -> Option<Self> {
        return serde_json::from_str::<Self>(identity_str).ok();
    }
    pub fn to_string(&self) -> Option<String> {
        return serde_json::to_string::<Self>(self).ok();
    }

    pub fn store(&self) -> bool {
        // We want to store the information for this in an encrypted fashion.
        println!("Saving to system.");
        let self_str = self.to_string().expect("Couldn't serialize this identity.");
        return set_secure(self.user.url.as_str(), self_str.as_str());
    }
}

pub fn network_for(url: &str) -> Option<Box<Authenticator>> {
    if MastodonAccount::supported(url) {
        println!("Building a Mastodon account handler.");
        return MastodonAccount::build_for(url);
    } else if IndieAuthAccount::supported(url) {
        println!("Building a IndieWeb account handler.");
        return IndieAuthAccount::build_for(url);
    } else {
        return None;
    }
}
