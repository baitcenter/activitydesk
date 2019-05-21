use crate::indieweb::indieauth;
use crate::mastodon;

pub trait AuthenticableAccount {
    fn get_authorization_url(&self) -> Option<String>;
    fn get_authentication_token(&self, code: &str) -> Option<String>;
}

struct DummyAccount {}

impl DummyAccount {
    pub fn new() -> Self {
        Self {}
    }
}

impl AuthenticableAccount for DummyAccount {
    fn get_authentication_token(&self, _code: &str) -> Option<String> {
        None
    }
    fn get_authorization_url(&self) -> Option<String> {
        None
    }
}

pub fn build_for(service_type: &str, profile_url: &str) -> Box<AuthenticableAccount> {
    return Box::new(mastodon::account::for_user(profile_url));
}

pub fn resolve_service_type(url: &str) -> Option<String> {
    if indieauth::supported(url) {
        return Some("indieweb".into());
    } else if mastodon::supported(url)? {
        return Some("mastodon".into());
    } else {
        return Some("unknown".into());
    }
}

pub fn get_dummy_account() -> Box<AuthenticableAccount> {
    return Box::new(DummyAccount::new());
}
