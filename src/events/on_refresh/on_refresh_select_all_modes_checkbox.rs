use crate::AppControls;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_select_all_modes_checkbox(
    app_controls_rc: Rc<RefCell<AppControls>>,
    has_step_file: bool,
    is_selected_all: bool,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let select_all_modes_checkbox = app_controls.get_select_all_modes_checkbox_mut();

    if has_step_file {
        select_all_modes_checkbox.enable();
    } else {
        select_all_modes_checkbox.disable();
    }
    select_all_modes_checkbox.set_checked(is_selected_all);
}
