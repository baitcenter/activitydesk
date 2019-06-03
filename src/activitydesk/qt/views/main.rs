use crate::activitydesk::account::Identity;
use crate::activitydesk::stream::get_all_sinks;
use qmetaobject::*;

#[derive(Default, QObject)]
pub struct Handler {
    base: qt_base_class!(trait QObject),
    register_new_account: qt_method!(fn(&mut self, result: String) -> ()),
    wipe_it: qt_method!(fn(&self)),
    set_current_identity: qt_method!(fn(&self, identity_url: String)),
    current_identity_changed: qt_signal!(),
    present_new_stream: qt_signal!(identity_url: String, stream_kind: String),
    current_identity_url: qt_property!(QString; READ current_identity_url NOTIFY current_identity_changed),
    identities: Vec<Identity>,
    current_identity: Identity,
    setup: qt_method!(fn(&mut self)),
}

impl Handler {
    pub fn wipe_it(&self) {
        crate::activitydesk::settings::wipe()
    }

    pub fn current_identity_url(&self) -> QString {
        self.current_identity.user.url.clone().into()
    }

    pub fn setup(&mut self) {
        match crate::activitydesk::settings::list_all_secure() {
            Some(ids) => self.identities = ids,
            _ => self.identities = vec![],
        }

        for sink in get_all_sinks() {
            match sink {
                Some(solid_sink) => {
                    // TODO: Emit sink_added signal.
                    let identity_url = solid_sink.identity().user.url;
                    let stream_kind = solid_sink.kind();
                    self.present_new_stream(identity_url, stream_kind);
                }
                _ => {}
            }
        }
    }

    pub fn set_current_identity(&mut self, identity_url: String) {
        self.current_identity = match self
            .identities
            .iter()
            .find(|id| id.user.url == identity_url)
        {
            Some(id) => id.clone(),
            _ => Identity::default(),
        };
        self.current_identity_changed();
    }

    pub fn register_new_account(&mut self, identity_str: String) {
        match Identity::from_string(identity_str.as_str()) {
            Some(identity) => {
                println!("Obtained a usable identity: {:?}", identity);
                identity.store();
            }
            _ => {}
        }
    }
}
