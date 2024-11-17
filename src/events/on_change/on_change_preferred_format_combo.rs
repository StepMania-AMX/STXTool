use crate::{on_refresh_preferred_format_combo, AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_change_preferred_format_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    preferred_format_index: i32,
) {
    let mut app_state = app_state_rc.borrow_mut();
    let prev_preferred_format_index = app_state.get_preferred_format_index().unwrap_or(-1);
    if prev_preferred_format_index != preferred_format_index {
        app_state.set_preferred_format_index(preferred_format_index);
    }
    drop(app_state);

    on_refresh_preferred_format_combo(app_controls_rc.clone(), app_state_rc.clone());
}
