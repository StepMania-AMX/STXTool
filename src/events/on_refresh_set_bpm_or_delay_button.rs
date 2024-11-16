use crate::{AppControls, AppState};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_set_bpm_or_delay_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    _table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let set_bpm_or_delay_button = app_controls.get_set_bpm_or_delay_button_mut();

    let app_state = app_state_rc.borrow();
    if let Some(_) = app_state.get_step_file() {
        set_bpm_or_delay_button.enable();
    } else {
        set_bpm_or_delay_button.disable();
    }
}
