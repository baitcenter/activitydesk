use crate::activitydesk::account::Identity;
use secret_service::{EncryptionType, SecretService};

static SECRET_SERVICE_KEY: &'static str = "ActivityDesk Credentials for";

fn service() -> SecretService {
    return SecretService::new(EncryptionType::Dh)
        .expect("Failed to grab handle to secret service.");
}

pub fn set_secure(name: &str, value: &str) -> bool {
    return service()
        .get_default_collection()
        .expect("Couldn't get default.")
        .create_item(
            [SECRET_SERVICE_KEY, name].join(" ").as_str(),
            Vec::default(),
            value.as_bytes(),
            false,
            "application/json",
        )
        .is_ok();
}

pub fn list_all_secure() -> Option<Vec<Identity>> {
    return match service()
        .get_default_collection()
        .expect("No collection.")
        .get_all_items()
    {
        Err(err) => {
            println!("Couldn't load items: {:?}", err);
            return None;
        }
        Ok(items) => {
            return Some(
                items
                    .iter()
                    .filter(|item| {
                        return item
                            .get_label()
                            .unwrap_or(String::default())
                            .starts_with(SECRET_SERVICE_KEY);
                    })
                    .map(|item| {
                        return Identity::from_string(
                            String::from_utf8(item.get_secret().expect("welp"))
                                .expect("works")
                                .as_str(),
                        )
                        .expect("Here it works");
                    })
                    .collect(),
            );
        }
    };
}
