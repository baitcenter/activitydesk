use crate::activitydesk::account::Identity;
use crate::activitydesk::publisher::*;
use elefren::http_send::HttpSender;
use elefren::Data;
use elefren::Mastodon;
use elefren::MastodonClient;
use elefren::StatusBuilder;

pub struct Composer {
    pub identity: Identity,
}

impl Composer {
    fn get_app(&self) -> Mastodon<HttpSender> {
        let masto_data: Data = serde_json::from_str(self.identity.access_data.as_str())
            .ok()
            .expect("Failed to deserialize Mastodon authentication information.");
        Mastodon::from(masto_data)
    }
}

impl Publisher for Composer {
    fn upload_media(&self, _media: &Media) -> Option<Status> {
        None
    }

    fn send(&self, post: &Post) -> Option<Status> {
        let masto_app = self.get_app();
        let status = StatusBuilder::new()
            .status(post.content.clone())
            .spoiler_text(post.summary.clone())
            .sensitive(post.sensitive)
            .language(elefren::Language::Eng)
            .build()
            .ok()
            .expect("Failed to build new post.");

        match masto_app.new_status(status) {
            Ok(_) => Some(Status::SuccessfulSend),
            _ => None,
        }
    }
}

impl Builder for Composer {
    fn build(identity: &Identity) -> Option<Box<Publisher>> {
        Some(Box::new(Composer {
            identity: identity.clone(),
        }))
    }
}
