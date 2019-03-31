extern crate rust_qt_binding_generator;

use rust_qt_binding_generator::build::QtModule;

fn main() {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    rust_qt_binding_generator::build::require_qt_version(5, 11, 0);
    rust_qt_binding_generator::build::Build::new(&out_dir)
        .bindings("bindings.json")
        .qrc("src/qml.qrc")
        .cpp("src/main.cpp")
        .module(QtModule::Gui)
        .module(QtModule::Qml)
        .compile("activitydesk");
}
