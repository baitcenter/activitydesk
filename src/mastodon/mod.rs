use crate::activitydesk::get_base_domain;
use elefren::apps::App;
use elefren::apps::AppBuilder;
use elefren::scopes::Scopes;
pub mod account;
pub mod http;

pub fn app() -> App {
    let mut app_builder: AppBuilder = App::builder();
    app_builder.client_name("ActivityDesk");
    app_builder.website("https://activitydesk.black.af");
    app_builder.scopes(Scopes::all());

    return match app_builder.build() {
        Ok(built_app) => built_app,
        _ => App::default(),
    };
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
