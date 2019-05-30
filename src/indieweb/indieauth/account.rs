use crate::activitydesk::account::*;
use crate::activitydesk::http;
use crate::indieweb::link_rel::extract_from_url;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::percent_encoding::percent_decode;
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TokenEndpointResponse {
    pub access_token: String,
    pub token_type: String,
    pub me: String,
    pub scope: String,
}

#[derive(Default)]
pub struct Account {
    pub url: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub access_token: String,
    state: Uuid,
    token: Option<TokenEndpointResponse>,
}

impl Account {
    fn form_authorization_url(&self) -> Option<String> {
        let rels = extract_from_url(self.url.as_str());
        let endpoint = rels.get("authorization_endpoint".into())?.first().unwrap();
        let scope_str = "read create follow mute block channels";
        return match Url::parse_with_params(
            endpoint,
            [
                ("client_id", "https://activitydesk.black.af"),
                ("me", self.url.as_str()),
                ("redirect_uri", "https://activitydesk.black.af/api/~present"),
                ("response_type", "code"),
                ("scope", scope_str),
                ("state", self.state.to_string().as_str()),
            ]
            .iter(),
        ) {
            Ok(url) => Some(url.as_str().into()),
            _ => None,
        };
    }

    fn resolve_token_endpoint(&self) -> Option<Url> {
        let rels = extract_from_url(self.url.as_str());
        return match rels.get("token_endpoint".into()) {
            None => None,
            Some(endpoint_strs) => {
                let endpoint_str = endpoint_strs
                    .first()
                    .expect("No token endpoints were available.");
                return Url::parse(endpoint_str).ok();
            }
        };
    }
}

impl Authenticator for Account {
    fn get_authentication_url(&self) -> Option<String> {
        return self.form_authorization_url();
    }

    fn obtain_access(&mut self, code: &str) -> Option<String> {
        return match self.resolve_token_endpoint() {
            Some(token_url) => {
                println!("Got the token URL: {:#?}", token_url);
                let unencoded_code = percent_decode(code.as_bytes())
                    .decode_utf8()
                    .ok()?
                    .to_owned();
                let mut params = HashMap::new();
                params.insert("client_id", "https://activitydesk.black.af");
                params.insert("code", unencoded_code.as_ref());
                params.insert("grant_type", "authorization_code");
                params.insert("me", self.url.as_str());
                params.insert("redirect_uri", "https://activitydesk.black.af/api/~present");

                let client = http::client();
                let resp = client.post(token_url).form(&params).send();
                return match resp {
                    Err(_) => None,
                    Ok(mut response) => {
                        println!("Got response {:#?}", response);
                        return match response.status() {
                            StatusCode::OK => {
                                let token_resp: TokenEndpointResponse = response.json().ok()?;
                                self.token = Some(token_resp.clone());
                                println!("{:#?}", token_resp);
                                return Some(token_resp.access_token);
                            }
                            _ => None,
                        };
                    }
                };
            }
            _ => None,
        };
    }

    fn generate_access_info(&self) -> Option<String> {
        return serde_json::to_string(&self.token).ok();
    }

    fn network_type(&self) -> String {
        "indieweb".into()
    }

    fn resolve_user(&self) -> User {
        return self.into();
    }
}

impl Builder for Account {
    fn supported(site_url: &str) -> bool {
        let rels = extract_from_url(site_url);
        println!("IndieAuth::Account::supported: {:#?}", rels);
        return rels.contains_key("authorization_endpoint") & rels.contains_key("token_endpoint");
    }

    fn build_for(site_url: &str) -> Option<Box<Authenticator>> {
        let rels = extract_from_url(site_url);
        let auth_endpoint = rels
            .get("authorization_endpoint".into())?
            .first()
            .expect("No IndieAuth authorization endpoint was powered.");
        let token_endpoint = rels
            .get("token_endpoint".into())?
            .first()
            .expect("No IndieAuth token endpoint was powered.");

        return Some(Box::new(Account {
            url: String::from(site_url),
            access_token: String::default(),
            authorization_endpoint: auth_endpoint.clone(),
            token_endpoint: token_endpoint.clone(),
            state: Uuid::new_v3(&Uuid::NAMESPACE_URL, site_url.as_bytes()),
            token: None,
        }));
    }
}

// TODO: Extract h-card information for this.
impl From<&Account> for User {
    fn from(account: &Account) -> User {
        return match account.token {
            Some(ref token_resp) => User {
                username: token_resp.me.clone(),
                url: token_resp.me.clone(),
                service_url: token_resp.me.clone(),
                image_url: token_resp.me.clone(),
            },
            _ => User::default(),
        };
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
            .get_authentication_url()
            .unwrap_or("http://invalid".into());
        assert!(obtained_url.starts_with("https://v2.jacky.wtf/indie/auth"));
    }
}
