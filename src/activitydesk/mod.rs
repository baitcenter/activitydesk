pub mod account;
pub mod qt;
pub mod settings;
pub mod stream;

pub fn base_user_agent() -> &'static str {
    return "ActivityDesk/0.0.1-dev (https://activitydesk.black.af)";
}

pub fn get_base_domain(url: &str) -> Option<String> {
    let parsed_url = url::Url::parse(url);
    let unwrapped_url = parsed_url.unwrap();
    let hostname: &str = unwrapped_url.host_str()?;
    let scheme: String = unwrapped_url.scheme().into();
    return Some(scheme + "://".into() + hostname);
}

pub fn init() {
    stream::init();
}
