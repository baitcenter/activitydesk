use crate::activitydesk::account::resolver::get_type_for_profile_url;
use crate::indieweb::indieauth as indieweb_account;
use crate::mastodon::account as mastodon_account;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),

    resolve_profile_kind: qt_method!(fn resolve_profile_kind(&mut self, url: String) -> QString {
        return match get_type_for_profile_url(url.as_str()) {
            Some(result) => QString::from(result),
            _ => QString::from("unknown")
        };
    }),

    get_auth_url: qt_method!(fn get_auth_url(&mut self, site_type: String, url: String) -> QString {
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
}
