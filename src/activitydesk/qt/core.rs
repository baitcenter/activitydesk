use crate::activitydesk::qt::views;
use cstr::*;
use qmetaobject::*;

// TODO: Add QApplication property here.
pub struct Inst {
    pub engine: QmlEngine,
}

impl Inst {
    pub fn new() -> Self {
        let mut engine = QmlEngine::new();
        engine.load_file("qrc:/qml/Main.qml".into());

        Self { engine: engine }
    }
}

pub fn setup() -> Inst {
    register_qml_types();
    return Inst::new();
}

pub fn run(inst: &mut Inst) {
    //! TODO: Add a singleton value to the app.
    inst.engine.exec();
}

fn register_qml_types() {
    qml_register_type::<views::new_account_dialog::Handler>(
        cstr!("af.black.activitydesk.handlers"),
        0,
        1,
        cstr!("NewAccountDialogHandler"),
    );
}
