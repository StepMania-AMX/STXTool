use crate::{AppControls, AppState, StepFormat};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_preferred_format_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    _table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let mut preferred_format_combo = app_controls.get_preferred_format_combo_mut();

    let app_state = app_state_rc.borrow();
    if let Some(_) = app_state.get_step_file() {
        preferred_format_combo.enable();
    } else {
        preferred_format_combo.disable();
    }
    preferred_format_combo.set_selected(StepFormat::Ucs as i32);
}
