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

// Provides logic for authenticating a user to their account.
pub trait Authenticator {
    fn resolve_authorization_url(&self) -> Option<String>;
    fn obtain_access_token(&mut self, code: &str) -> Option<String>;
    fn access_token(&self) -> Option<String>;
    fn network_type(&self) -> String;
    fn resolve_user(&self) -> User;
}

pub trait Builder {
    fn supported(url: &str) -> bool;
    fn build_for(url: &str) -> Option<Box<Authenticator>>;
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
