use crate::AppControls;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_set_bpm_or_delay_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    has_step_file: bool,
    is_selected_none: bool,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let set_bpm_or_delay_button = app_controls.get_set_bpm_or_delay_button_mut();

    if has_step_file && !is_selected_none {
        set_bpm_or_delay_button.enable();
    } else {
        set_bpm_or_delay_button.disable();
    }
}
