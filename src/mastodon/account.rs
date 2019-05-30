use super::app;
use crate::activitydesk::account::*;
use crate::activitydesk::get_base_domain;
use crate::activitydesk::server;
use elefren::http_send::HttpSender;
use elefren::registration::{Registered, Registration};
use elefren::{Mastodon, MastodonClient};
use reqwest::header;

#[derive(Default)]
pub struct Account {
    instance_url: String,
    access_token: Option<String>,
    registration: Option<Registered<HttpSender>>,
    api: Option<Mastodon<HttpSender>>,
}

impl Authenticator for Account {
    fn get_authentication_url(&self) -> Option<String> {
        let url_result = self.registration.as_ref()?.authorize_url();
        return match url_result {
            Ok(url) => Some(url),
            _ => None,
        };
    }

    fn obtain_access(&mut self, authorization_code: &str) -> Option<String> {
        return match self.registration.as_ref()?.complete(authorization_code) {
            Ok(app) => {
                println!("Successfully authenticated with Mastodon.");
                let data = app.data.clone();
                let token: String = data.token.into_owned().into();
                self.access_token = Some(token.clone());
                self.api = Some(app);
                return Some(token.clone());
            }
            _ => None,
        };
    }

    fn resolve_user(&self) -> User {
        return self.into();
    }

    fn network_type(&self) -> String {
        "mastodon".into()
    }

    fn generate_access_info(&self) -> Option<String> {
        return serde_json::to_string(&self.api.as_ref()?.data).ok();
    }
}

fn check_endpoint(url: &str) -> bool {
    match get_base_domain(url) {
        Some(instance_host) => {
            let instance_info_url: String = instance_host + "/api/v1/instance".into();
            println!(
                "Calling {:?} to get instance information...",
                instance_info_url
            );

            return match reqwest::get(instance_info_url.as_str()) {
                Ok(mut result) => {
                    if result.status().is_success() {
                        let resp_body = result.text().ok().expect("No body found.");
                        let resp_json = json::parse(&resp_body.as_str())
                            .ok()
                            .expect("No JSON information");

                        // TODO: Need to do a stricter check.
                        !resp_json["version"].is_null()
                    } else {
                        false
                    }
                }
                _ => false,
            };
        }
        _ => {
            eprintln!("Couldn't resolve a legit domain for {:?}", url);
            return false;
        }
    }
}

impl Builder for Account {
    fn supported(url: &str) -> bool {
        return check_endpoint(url) || server::get_name(url).contains("Mastodon");
    }

    fn build_for(url: &str) -> Option<Box<Authenticator>> {
        let instance_url = get_base_domain(url).unwrap();
        return match Registration::new(instance_url.clone()).register(app()) {
            Ok(reg) => Some(Box::new(Account {
                registration: Some(reg),
                api: None,
                access_token: None,
                instance_url,
            })),
            Err(err) => {
                println!("Failed to register application: {:?}", err);
                return None;
            }
        };
    }
}

impl From<&Account> for User {
    fn from(account: &Account) -> User {
        let masto_account = account
            .api
            .as_ref()
            .expect("Failed to grab API handle for authenticated Mastodon user.")
            .verify_credentials();

        return match masto_account {
            Ok(mastodon_account) => User {
                username: mastodon_account.acct,
                url: mastodon_account.url,
                service_url: account.instance_url.clone(),
                image_url: mastodon_account.avatar,
            },
            _ => User::default(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_impl_supported_test_figures_out_by_header() {
        assert!(Account::supported("https://mastodon.social/@blackaf"));
        assert!(Account::supported("https://mastodon.social"));
    }

    #[test]
    fn builder_impl_supported_test_fails_if_not_visible() {
        assert_eq!(Account::supported("https://black.af/"), false);
    }
}
