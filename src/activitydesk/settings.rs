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

pub fn wipe() {
    match service()
        .get_default_collection()
        .expect("No collection.")
        .get_all_items()
    {
        Err(err) => {
            println!("Couldn't load items: {:?}", err);
        }
        Ok(items) => {
            items
                .iter()
                .map(|item| {
                    if item.is_locked().ok().unwrap() == true {
                        item.unlock().ok().unwrap()
                    } else {
                        println!("Already unlocked.");
                    }

                    if item
                        .get_label()
                        .unwrap_or(String::default())
                        .starts_with(SECRET_SERVICE_KEY)
                    {
                        println!("Wiping.");
                        assert!(item.delete().is_ok());
                    }
                })
                .count();
        }
    }
}

pub fn list_all_secure() -> Option<Vec<Identity>> {
    match service()
        .get_default_collection()
        .expect("No collection.")
        .get_all_items()
    {
        Err(err) => {
            println!("Couldn't load items: {:?}", err);
            return None;
        }
        Ok(items) => {
            let filtered_items: Vec<Identity> = items
                .iter()
                .filter(|item| {
                    let label = item.get_label().unwrap_or(String::default());
                    label.starts_with(SECRET_SERVICE_KEY)
                })
                .map(|item| {
                    let identity_secret =
                        item.get_secret().expect("Failed to extract identity info.");
                    let identity_str = String::from_utf8(identity_secret)
                        .expect("Failed to convert identity info into a UTF-8 string.");
                    return match Identity::from_string(identity_str.as_str()) {
                        Some(identity) => identity,
                        None => {
                            println!("Could not deserialize the identity provided.");
                            return Identity::default();
                        }
                    };
                })
                .filter(|item| item.user.url != String::default())
                .collect();
            println!("Found {} identities.", filtered_items.len());
            return Some(filtered_items);
        }
    }
}
