use crate::mastodon::app;
use crate::mastodon::http;
use elefren::registration::{Registered, Registration};
use std::collections::HashMap;

lazy_static! {
    static ref REGISTRATIONS: HashMap<String, Registered<http::Sender>> = {
        let mut regs: HashMap<String, Registered<http::Sender>> = HashMap::new();
        return regs;
    };
}

fn register_for_instance(instance_url: &str) -> Option<&Registered<http::Sender>> {
    let mut registered_app =
        Registration::with_sender(instance_url, http::Sender {}).register(app());
    return match registered_app {
        Ok(reg) => {
            REGISTRATIONS.insert(instance_url.into(), reg);
            return Some(&reg);
        }
        _ => None,
    };
}

fn registration_for(instance_url: &str) -> Option<&Registered<http::Sender>> {
    return match REGISTRATIONS.get(instance_url.into()) {
        None => register_for_instance(instance_url),
        reg => reg,
    };
}

pub fn get_authorization_url(instance_url: &str) -> Option<String> {
    return match registration_for(instance_url) {
        Some(reg) => Some(reg.authorize_url().unwrap()),
        _ => None,
    };
}

pub fn get_authorization_token(instance_url: &str, authorization_code: &str) -> Option<String> {
    return match registration_for(instance_url) {
        Some(reg) => {
            return match reg.complete(authorization_code) {
                Ok(app) => Some(app.data.token.into()),
                _ => None,
            };
        }
        _ => None,
    };
}
