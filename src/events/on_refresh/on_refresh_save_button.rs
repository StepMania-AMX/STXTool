use crate::{AppControls, AppState};
use libamx::StxVersion;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_save_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let save_button = app_controls.get_save_button_mut();

    let app_state = app_state_rc.borrow();
    match app_state.get_step_file() {
        Some(stx_file)
            if stx_file.get_version() == StxVersion::NewXenesis
                || stx_file.get_path().as_os_str().is_empty() =>
        {
            save_button.disable();
        }
        Some(_) => {
            save_button.enable();
        }
        None => {
            save_button.disable();
        }
    }
}
