use elefren::prelude::Registration;

pub fn get_authorization_url(instance_url: &str) -> Option<String> {
    let app = crate::mastodon::app()?;
    match Registration::new(instance_url).register(app) {
        Ok(registration) => Some(registration.authorize_url().unwrap()),
        _ => None,
    }
}

pub fn get_authorization_token(instance_url: &str, authorization_code: &str) -> Option<String> {
    let app = crate::mastodon::app()?;
    match Registration::new(instance_url).register(app) {
        Ok(registration) => {
            return match registration.complete(authorization_code) {
                Ok(app) => Some(app.data.token.into()),
                _ => None,
            };
        }
        _ => None,
    }
}
