pub fn get_type_for_profile_url(url: &str) -> Option<String> {
    if crate::indieweb::indieauth::supported(url) {
        return Some("indieweb".to_string());
    } else if crate::mastodon::supported(url)? {
        return Some("mastodon".to_string());
    } else {
        return Some("unknown".to_string());
    }
}
