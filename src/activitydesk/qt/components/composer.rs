use crate::activitydesk::publisher;
use crate::activitydesk::qt::models;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    send: qt_method!(fn(&self) -> bool),
    clear: qt_method!(fn(&mut self)),
    identity: qt_property!(models::identity::Item; WRITE set_identity),
    message: qt_property!(QString; WRITE set_message READ get_message NOTIFY updated),
    summary: qt_property!(QString; WRITE set_summary READ get_summary NOTIFY updated),
    updated: qt_signal!(),

    item: Option<models::identity::Item>,
    text: String,
    subject: String,
}

impl Handler {
    pub fn send(&self) -> bool {
        println!("Sending {:#?} via {:#?}", self.text, self.item);
        let current_identity = self.item.clone().unwrap().handle;
        let post = publisher::Post {
            summary: self.subject.clone(),
            content: self.text.clone(),
            sensitive: true,
        };
        publisher::async_send(current_identity, post);
        true
    }

    pub fn set_identity(&mut self, new_identity: models::identity::Item) {
        println!("Setting identity to {:#?}", new_identity);
        self.item = Some(new_identity.clone())
    }

    pub fn set_message(&mut self, new_message: QString) {
        self.text = new_message.into()
    }

    pub fn set_summary(&mut self, new_summary: QString) {
        self.subject = new_summary.into()
    }

    pub fn get_summary(&self) -> QString {
        self.subject.clone().into()
    }

    pub fn get_message(&self) -> QString {
        self.text.clone().into()
    }

    pub fn clear(&mut self) {
        self.subject = String::default();
        self.text = String::default();
        self.updated();
    }
}
