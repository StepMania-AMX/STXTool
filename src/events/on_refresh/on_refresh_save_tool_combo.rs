use crate::{AppControls, AppState, StepAuthoringTool};
use libamx::StxVersion;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_save_tool_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let save_tool_combo = app_controls.get_save_tool_combo_mut();

    let app_state = app_state_rc.borrow();
    match app_state.get_step_file() {
        Some(stx_file) if stx_file.get_version() == StxVersion::Rebirth => {
            save_tool_combo.disable();
            save_tool_combo.set_value("");
        }
        Some(stx_file) => {
            save_tool_combo.enable();
            let save_tool_value = stx_file.get_header().created_by.get_created_by();
            save_tool_combo.set_value(if save_tool_value.is_empty() {
                StepAuthoringTool::StepEdit.into()
            } else {
                save_tool_value
            });
        }
        None => {
            save_tool_combo.disable();
            save_tool_combo.set_value("");
        }
    }
}
