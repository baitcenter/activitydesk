use crate::activitydesk::account::Identity;
use crate::activitydesk::http;
use crate::activitydesk::publisher::*;
use crate::indieweb::indieauth::TokenEndpointResponse;
use crate::indieweb::link_rel::extract_from_url;
use reqwest::header::LOCATION;
use reqwest::RequestBuilder;
use reqwest::StatusCode;
use std::collections::HashMap;

pub struct Composer {
    pub identity: Identity,
}

impl Composer {
    fn get_micropub_endpoint(&self) -> Option<String> {
        let rels = extract_from_url(self.identity.user.url.as_str());
        rels.get("micropub".into())?.first().cloned()
    }

    fn build_client_for(&self, url: &str) -> RequestBuilder {
        let auth_resp: TokenEndpointResponse =
            serde_json::from_str(self.identity.access_data.as_str())
                .ok()
                .expect("Failed to obtain account information.");
        let client = http::client();
        client.post(url).bearer_auth(auth_resp.access_token)
    }

    fn build_params_from_post(&self, post: &Post) -> HashMap<&str, String> {
        let mut params = HashMap::new();
        params.insert("h", "entry".into()); // TODO: Allow user to specify different one.
        params.insert("content", post.content.clone());

        if post.summary != String::default() {
            params.insert("summary", post.summary.clone());
        }

        params
    }
}

impl Publisher for Composer {
    fn upload_media(&self, _media: &Media) -> Option<Status> {
        None
    }

    fn send(&self, post: &Post) -> Option<Status> {
        let params = self.build_params_from_post(post);
        match self
            .build_client_for(self.get_micropub_endpoint()?.as_str())
            .form(&params)
            .send()
        {
            Err(err) => {
                println!("Error: {:#?}", err);
                Some(Status::FailedToSend)
            }
            Ok(mut response) => {
                println!("Got response {:#?}", response);
                let valid_responses =
                    vec![StatusCode::OK, StatusCode::CREATED, StatusCode::ACCEPTED];
                let status = response.status();
                if valid_responses.contains(&status) {
                    let entry_uri = response.headers().get(LOCATION);
                    println!("Post: {:#?}", entry_uri);
                    return Some(Status::SuccessfulSend);
                } else {
                    eprintln!(
                        "Got a {} back from the site: {:#?}",
                        status,
                        response.text()
                    );
                    return Some(Status::FailedToSend);
                }
            }
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
