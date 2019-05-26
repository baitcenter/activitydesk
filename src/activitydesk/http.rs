use reqwest::header::*;

pub fn user_agent() -> String {
    return "ActivityDesk/0.0.1-dev (https://activitydesk.black.af)".into();
}

fn headers() -> reqwest::header::HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        user_agent()
            .parse()
            .expect("Failed to inject default user-agent for ActivityDesk."),
    );
    return headers;
}

pub fn client() -> reqwest::Client {
    return reqwest::Client::builder()
        .default_headers(headers())
        .build()
        .ok()
        .expect("Failed to build HTTP Client.");
}
