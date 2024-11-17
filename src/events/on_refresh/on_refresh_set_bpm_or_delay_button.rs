use crate::{AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_set_bpm_or_delay_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state: &AppState,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let set_bpm_or_delay_button = app_controls.get_set_bpm_or_delay_button();

    if app_state.get_step_file().is_some() && !app_state.get_is_selected_none().unwrap_or(true) {
        set_bpm_or_delay_button.enable();
    } else {
        set_bpm_or_delay_button.disable();
    }
}
