use crate::activitydesk::get_base_domain;
use std::ops::Index;

pub fn get_name(url: &str) -> String {
    let nodeinfo = get_nodeinfo(url);
    if nodeinfo.is_some() {
        return nodeinfo.unwrap();
    }

    String::default()
}

fn get_nodeinfo(url: &str) -> Option<String> {
    match get_base_domain(url) {
        Some(instance_host) => {
            let webfinger_url: String = instance_host + "/.well-known/nodeinfo".into();
            let resp_text = reqwest::get(webfinger_url.as_str()).ok()?.text().ok()?;
            let nodeinfo_versions = json::parse(&resp_text).ok()?.index("links").clone();
            let nodeinfo_url: Option<String> = if nodeinfo_versions.is_null() {
                None
            } else {
                println!("{:#?}", nodeinfo_versions);
                let nodeinfo_schema_urls: Vec<String> = nodeinfo_versions
                    .members()
                    .map(|value| {
                        println!("{:#?}", value);
                        match value["href"].clone().as_str() {
                            Some(href) => href.into(),
                            _ => "".into(),
                        }
                    })
                    .collect();
                if nodeinfo_schema_urls.is_empty() {
                    None
                } else {
                    nodeinfo_schema_urls.first().cloned()
                }
            };

            let nodeinfo_text = reqwest::get(nodeinfo_url.unwrap().as_str())
                .ok()?
                .text()
                .ok()?;
            let nodeinfo = json::parse(&nodeinfo_text).ok()?;
            let software = nodeinfo["software"]["name"].clone();

            if software.is_null() {
                None
            } else {
                Some(software.as_str()?.into())
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_nodeinfo_test() {
        let koype_result = get_nodeinfo("https://v2.jacky.wtf");
        assert!(koype_result.is_some());
        assert_eq!(koype_result.unwrap(), "Koype");
        assert!(get_nodeinfo("https://jacky.wtf").is_none());
    }
}
