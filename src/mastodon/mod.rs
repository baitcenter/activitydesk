pub mod account;
pub mod http;
pub mod publisher;
pub mod stream;

use elefren::apps::{App, AppBuilder};
use elefren::scopes::Scopes;

pub fn app() -> App {
    let mut app_builder: AppBuilder = App::builder();
    app_builder.client_name("ActivityDesk");
    app_builder.website("https://activitydesk.black.af");
    app_builder.scopes(Scopes::all());

    return match app_builder.build() {
        Ok(built_app) => built_app,
        Err(err) => {
            println!("Failed to build app for registration: {:?}", err);
            return App::default();
        }
    };
}
