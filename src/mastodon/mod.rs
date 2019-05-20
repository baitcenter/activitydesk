use crate::activitydesk::get_base_domain;
use elefren::apps::App;
pub mod account;

pub fn app() -> Option<App> {
    let mut app = App::builder();
    app.client_name("ActivityDesk");
    return Some(app.build().unwrap());
}

pub fn supported(url: &str) -> Option<bool> {
    let instance_host = get_base_domain(url)?;
    let instance_info_url: String = instance_host + "/api/v1/instance".into();
    let result = reqwest::get(instance_info_url.as_str());

    if result.is_ok() {
        if result.unwrap().headers().get("Server").unwrap() == "Mastodon" {
            return Some(true);
        }
    }

    return Some(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_test_figures_out_by_header() {
        match supported("https://mastodon.technology/@blackaf") {
            Some(result) => assert!(result),
            _ => assert!(false),
        }
    }

    #[test]
    fn supported_test_fails_if_not_visible() {
        match supported("https://koype.net/") {
            Some(result) => assert!(!result),
            _ => assert!(false),
        }
    }
}
