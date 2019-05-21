mod activitydesk;
mod indieweb;
mod mastodon;

use qmetaobject::*;

// NOTE: This _has_ to be done here - fails if moved into another module
// TODO: Move call below to activitydesk::qt::load_resources()
qrc! {
    qml_resources,
    "/" {
        "src/activitydesk/qml/main.qml" as "qml/Main.qml",
        "src/activitydesk/qml/accounts/new.qml" as "qml/Accounts/New.qml",
    }
}

fn main() {
    qml_resources();
    let mut qt = activitydesk::qt::core::setup();
    activitydesk::qt::core::run(&mut qt);
}
