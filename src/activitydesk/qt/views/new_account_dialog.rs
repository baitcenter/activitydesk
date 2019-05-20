use crate::activitydesk::account::resolver::get_type_for_profile_url;
use crate::indieweb::indieauth as indieweb_account;
use crate::mastodon::account as mastodon_account;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    profile_url: String,
    kind: String,
    base: qt_base_class!(trait QObject),

    resolve_profile_kind: qt_method!(fn resolve_profile_kind(&mut self, url: String) -> QString {
        return match get_type_for_profile_url(url.as_str()) {
            Some(result) => QString::from(result),
            _ => QString::from("unknown")
        };
    }),

    get_auth_url: qt_method!(fn get_auth_url(&mut self, site_type: String, url: String) -> QString {
        self.profile_url = String::from(url.as_str());
        self.kind = String::from(site_type.as_str());
        if (site_type == "indieweb") & indieweb_account::supported(url.as_str()) {
            return match indieweb_account::get_authorization_request_url(url.as_str()) {
                None => QString::from(""),
                Some(auth_url) => QString::from(auth_url),
            }
        } else if site_type == "mastodon" {
            let instance_host = crate::activitydesk::get_base_domain(url.as_str());
            return match mastodon_account::get_authorization_url(instance_host.unwrap().as_str()) {
                None => QString::from(""),
                Some(auth_url) => QString::from(auth_url),
            }
        } else {
            return QString::from("");
        }
    }),

    get_auth_token: qt_method!(fn get_auth_token(&mut self, authorization_code: String) -> QString {
        return match self.kind.as_str() {
            "mastodon" => {
                let instance_host = crate::activitydesk::get_base_domain(self.profile_url.as_str());
                return match mastodon_account::get_authorization_token(instance_host.unwrap().as_str(), authorization_code.as_str()) {
                    None => QString::from(""),
                    Some(token) => QString::from(token),
                };
            }
            _ => QString::from(""),
        }
    }),
}
