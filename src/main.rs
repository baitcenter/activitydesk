extern crate hyper;
extern crate secret_service;

mod activitydesk;
mod indieweb;
mod mastodon;

use qmetaobject::*;

// NOTE: This _has_ to be done here - fails if moved into another module
// TODO: Move call below to activitydesk::qt::load_resources()
qrc! {
    qml_resources,
    "/" {
        "src/activitydesk/qml/components/stream.qml" as "qml/Components/Stream.qml",
        "src/activitydesk/qml/components/post.qml" as "qml/Components/Post.qml",
        "src/activitydesk/qml/components/composer.qml" as "qml/Components/Composer.qml",
        "src/activitydesk/qml/components/account_chooser.qml" as "qml/Components/AccountChooser.qml",
        "src/activitydesk/qml/accounts/new.qml" as "qml/Accounts/New.qml",
        "src/activitydesk/qml/main.qml" as "qml/Main.qml",
    }
}

fn main() {
    qml_resources();
    activitydesk::init();
    activitydesk::qt::core::start();
}
