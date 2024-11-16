use crate::{AppControls, AppState};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_save_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    _table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let mut save_button = app_controls.get_save_button_mut();

    let app_state = app_state_rc.borrow();
    if let Some(stx_file) = app_state.get_step_file() {
        if stx_file.get_path().as_os_str().is_empty() {
            save_button.disable();
        } else {
            save_button.enable();
        }
    } else {
        save_button.disable();
    }
}
