use crate::interface::{SessionModelEmitter, SessionModelTrait};

pub struct SessionModel {
    emit: SessionModelEmitter,
}

impl SessionModel {}
impl SessionModelTrait for SessionModel {
    fn new(mut emit: SessionModelEmitter) -> SessionModel {
        SessionModel { emit: emit.clone() }
    }
    fn emit(&mut self) -> &mut SessionModelEmitter {
        &mut self.emit
    }
}
