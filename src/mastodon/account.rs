use elefren::prelude::Registration;

pub fn get_authorization_url(instance_url: &str) -> Option<String> {
    let app = crate::mastodon::app()?;
    match Registration::new(instance_url).register(app) {
        Ok(registration) => Some(registration.authorize_url().unwrap()),
        _ => None,
    }
}
