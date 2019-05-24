use crate::activitydesk::account::{Authenticator, Builder, User};
use crate::indieweb::link_rel::extract_from_resp;
use reqwest::Url;
use uuid::Uuid;

#[derive(Default)]
pub struct Account {
    pub url: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub access_token: String,
    state: Uuid,
}

impl Authenticator for Account {
    fn resolve_authorization_url(&self) -> Option<String> {
        let resp = reqwest::get(self.url.as_str());
        let rels = extract_from_resp(resp);
        let endpoint = rels.get("authorization_endpoint".into())?.first().unwrap();
        return match Url::parse_with_params(
            endpoint,
            [
                ("client_id", "https://activitydesk.black.af"),
                ("state", self.state.to_string().as_str()),
                ("response_type", "code"),
                ("redirect_uri", "https://activitydesk.black.af/api/~present"),
                ("me", self.url.as_str()),
                ("scope", "read"),
            ]
            .iter(),
        ) {
            Ok(url) => Some(url.as_str().into()),
            _ => None,
        };
    }

    fn obtain_access_token(&mut self, _code: &str) -> Option<String> {
        // TODO: Build token endpoint URI.
        // TODO: Send request and extract response.
        None
    }

    fn access_token(&self) -> Option<String> {
        None
    }

    fn network_type(&self) -> String {
        "indieweb".into()
    }

    fn resolve_user(&self) -> User {
        User::default()
    }
}

impl Builder for Account {
    fn supported(site_url: &str) -> bool {
        let rels = extract_from_resp(reqwest::get(site_url));
        return rels.contains_key("authorization_endpoint") & rels.contains_key("token_endpoint");
    }

    fn build_for(site_url: &str) -> Option<Box<Authenticator>> {
        let resp = reqwest::get(site_url);
        let rels = extract_from_resp(resp);
        let auth_endpoint = rels.get("authorization_endpoint".into())?.first();
        let token_endpoint = rels.get("token_endpoint".into())?.first();

        return Some(Box::new(Account {
            url: String::from(site_url),
            access_token: String::default(),
            authorization_endpoint: auth_endpoint.unwrap().clone(),
            token_endpoint: token_endpoint.unwrap().clone(),
            state: Uuid::new_v3(&Uuid::NAMESPACE_URL, site_url.as_bytes()),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_resolve_authorization_url_test() {
        let mut acc = Account::default();
        acc.url = "https://v2.jacky.wtf".into();
        acc.state = Uuid::new_v3(&Uuid::NAMESPACE_URL, "https://v2.jacky.wtf".as_bytes());
        let obtained_url = acc
            .resolve_authorization_url()
            .unwrap_or("http://invalid".into());
        assert!(obtained_url.starts_with("https://v2.jacky.wtf/indie/auth"));
    }
}
