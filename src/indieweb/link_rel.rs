use crate::activitydesk::http;
use reqwest::{header::HeaderMap, header::LINK, Response, Result};
use soup::*;
use std::collections::HashMap;

pub fn extract_from_url(url: &str) -> HashMap<String, Vec<String>> {
    let client = http::client();
    let resp = client.get(url).send();
    println!("extract_from_url: {:#?}", resp);
    return extract_from_resp(resp);
}

pub fn extract_from_resp(resp: Result<Response>) -> HashMap<String, Vec<String>> {
    match resp {
        Ok(mut resp) => {
            let mut rels = extract_from_headers(resp.headers().clone());
            for (rel, value) in extract_from_html(resp.text().unwrap().as_str()) {
                if rels.contains_key(&rel) {
                    rels.get_mut(&rel).unwrap().extend(value);
                } else {
                    rels.insert(rel, value);
                }
            }
            println!("Got rels: {:#?}", rels);
            return rels;
        }
        Err(err) => {
            println!("Failed to fetch rel info for response: {:#?}", err);
            HashMap::new()
        }
    }
}

pub fn extract_from_headers(headers: HeaderMap) -> HashMap<String, Vec<String>> {
    let mut rels: HashMap<String, Vec<String>> = HashMap::new();
    for link_header_value in headers.get_all(LINK).iter() {
        let link_values: Vec<&str> = link_header_value.to_str().unwrap().split(", ").collect();
        for link_value in link_values {
            let mut link_rel: Vec<&str> = link_value
                .trim()
                .split("; ")
                .map(|value| value.trim())
                .collect();

            let url_wrapper_chars: &[_] = &['>', '<'];
            let url = link_rel.remove(0).trim_matches(url_wrapper_chars);
            let mut attrs: HashMap<String, String> = HashMap::new();

            for attr in link_rel {
                let mut attr_components: Vec<&str> = attr.split('=').collect();
                let attr_name = attr_components.remove(0);
                let attr_value = attr_components.join("=");
                attrs.insert(attr_name.into(), attr_value.trim_matches('"').into());
            }

            let rel_name = &attrs["rel"];
            let urls: Vec<String> = vec![url.into()];
            if rels.contains_key(rel_name) {
                rels.get_mut(rel_name).unwrap().extend(urls);
            } else {
                rels.insert(rel_name.clone(), urls);
            }
        }
    }

    return rels;
}

pub fn extract_from_html(html: &str) -> HashMap<String, Vec<String>> {
    let soup = Soup::new(html);
    let acc: HashMap<String, Vec<String>> = HashMap::new();

    return soup.tag("link").find_all().fold(acc, |mut acc, elem| {
        let link_relname = elem.get("rel").unwrap();
        let link_href = elem.get("href").unwrap();
        if acc.contains_key(&link_relname) {
            acc.get_mut(&link_relname).unwrap().extend(vec![link_href]);
        } else {
            acc.insert(link_relname, vec![link_href]);
        }
        return acc;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_from_html_test_builds_list() {
        let html = r#"
    <link rel="test" href="value">
    "#;

        let results = extract_from_html(html);
        assert_eq!(results["test"], ["value"]);
    }

    #[test]
    fn extract_from_html_test_builds_multiple_values() {
        let html = r#"
    <link rel="test" href="value">
    <link rel="test" href="value-2">
    "#;

        let results = extract_from_html(html);
        assert_eq!(results["test"], ["value", "value-2"]);
    }

    #[test]
    fn extract_from_headers_builds_list() {
        let mut headers = HeaderMap::new();
        headers.insert(LINK, "<https://mastodon.technology/.well-known/webfinger?resource=acct%3Ablackaf%40mastodon.technology>; rel=\"lrdd\"; type=\"application/xrd+xml\"".parse().unwrap());

        let rels = extract_from_headers(headers);
        assert!(rels["lrdd"].contains(&"https://mastodon.technology/.well-known/webfinger?resource=acct%3Ablackaf%40mastodon.technology".into()));
    }

    #[test]
    fn extract_from_headers_builds_multiple_values() {
        let mut headers = HeaderMap::new();
        headers.insert(LINK, "<https://mastodon.technology/.well-known/webfinger?resource=acct%3Ablackaf%40mastodon.technology>; rel=\"lrdd\"; type=\"application/xrd+xml\", <https://mastodon.technology/users/blackaf.atom>; rel=\"alternate\"; type=\"application/atom+xml\", <https://mastodon.technology/users/blackaf>; rel=\"alternate\"; type=\"application/activity+json\"".parse().unwrap());

        let rels = extract_from_headers(headers);
        assert_eq!(rels["lrdd"], ["https://mastodon.technology/.well-known/webfinger?resource=acct%3Ablackaf%40mastodon.technology"]);
        assert_eq!(
            rels["alternate"],
            [
                "https://mastodon.technology/users/blackaf.atom",
                "https://mastodon.technology/users/blackaf"
            ]
        );
    }
}
