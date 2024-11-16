use crate::{AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_version_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let version_combo = app_controls.get_version_combo_mut();

    let app_state = app_state_rc.borrow();
    if let Some(stx_file) = app_state.get_step_file() {
        version_combo.enable();
        version_combo.set_selected(stx_file.get_version() as i32);
    } else {
        version_combo.disable();
        version_combo.set_selected(-1);
    }
}
