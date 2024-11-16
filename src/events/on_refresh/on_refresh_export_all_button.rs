use crate::{AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_export_all_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let export_all_button = app_controls.get_export_all_button_mut();

    let app_state = app_state_rc.borrow();
    if let Some(_) = app_state.get_step_file() {
        export_all_button.enable();
    } else {
        export_all_button.disable();
    }
}
