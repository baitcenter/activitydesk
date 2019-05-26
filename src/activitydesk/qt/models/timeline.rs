use qmetaobject::*;
use std::collections::HashMap;

#[derive(Default, QObject)]
pub struct Model {
    base: qt_base_class!(trait QAbstractListModel),
}

impl QAbstractListModel for Model {
    fn row_count(&self) -> i32 {
        0
    }

    fn data(&self, _index: QModelIndex, _role: i32) -> QVariant {
        return QVariant::default();
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        return HashMap::new();
    }
}
