use crate::{AppControls, AppState, StepFormat};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_preferred_format_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let preferred_format_combo = app_controls.get_preferred_format_combo();

    let app_state = app_state_rc.borrow_mut();
    if let Some(preferred_format_index) = app_state.get_preferred_format_index() {
        preferred_format_combo.enable();
        preferred_format_combo.set_selected(preferred_format_index);
    } else {
        preferred_format_combo.disable();
        preferred_format_combo.set_selected(StepFormat::EXPORT_DEFAULT_INDEX as i32);
    }
}
