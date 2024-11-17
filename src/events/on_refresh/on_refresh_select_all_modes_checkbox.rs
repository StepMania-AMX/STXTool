use crate::{AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_select_all_modes_checkbox(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state: &AppState,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let select_all_modes_checkbox = app_controls.get_select_all_modes_checkbox();

    if app_state.get_step_file().is_some() {
        select_all_modes_checkbox.enable();
    } else {
        select_all_modes_checkbox.disable();
    }
    select_all_modes_checkbox.set_checked(app_state.get_is_selected_all().unwrap_or(false));
}
