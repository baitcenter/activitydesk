use soup::*;
use std::collections::HashMap;

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
}
