use super::link_rel::*;

pub fn supported(site_url: &str) -> bool {
    return match reqwest::get(site_url) {
        Ok(mut resp) => {
            let rels = extract_from_html(resp.text().unwrap().as_str());
            return rels.contains_key("authorization_endpoint")
                & rels.contains_key("token_endpoint");
        }
        Err(_) => false,
    };
}

pub fn get_authorization_request_url(site_url: &str) -> Option<String> {
    return match reqwest::get(site_url) {
        Ok(mut resp) => {
            let rels = extract_from_html(resp.text().unwrap().as_str());
            return Some(rels.get("authorization_endpoint")?.first()?.as_str().into());
        }
        Err(_) => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_test_checks_if_authorization_endpoint_exists() {
        assert_eq!(supported("https://jacky.wtf"), true);
        assert_eq!(supported("https://mozilla.org"), false);
    }
}
