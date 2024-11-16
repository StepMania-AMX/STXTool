use libamx::LegacyMode;
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_mode_table_row(table_model_rc: Rc<RefCell<TableModel>>, mode: LegacyMode) {
    let table_model = table_model_rc.borrow_mut();
    table_model.notify_row_changed(mode as i32);
}
