use crate::activitydesk::qt;
use cstr::*;
use qmetaobject::*;

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
    inst.engine.exec();
}

fn register_qml_types() {
    qml_register_type::<qt::views::new_account_dialog::Handler>(
        cstr!("af.black.activitydesk.handlers"),
        0,
        1,
        cstr!("NewAccountDialogHandler"),
    );
    qml_register_type::<qt::views::main::Handler>(
        cstr!("af.black.activitydesk.handlers"),
        0,
        1,
        cstr!("MainWindowHandler"),
    );
}
