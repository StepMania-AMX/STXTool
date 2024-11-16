use crate::{on_refresh_save_tool_combo, AppControls, AppState, StepAuthoringTool};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_change_save_tool_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    save_tool: String,
) {
    let app_state = app_state_rc.borrow();
    let prev_save_tool = app_state
        .get_step_file()
        .map(|step_file| {
            step_file
                .get_header()
                .created_by
                .get_created_by()
                .to_string()
        })
        .unwrap();
    drop(app_state);

    let mut app_state = app_state_rc.borrow_mut();
    let save_tool =
        if save_tool == <StepAuthoringTool as Into<&str>>::into(StepAuthoringTool::StepEdit) {
            String::new()
        } else {
            save_tool
        };

    match app_state.get_step_file_mut() {
        Some(stx_file) => {
            if prev_save_tool != save_tool {
                let truncated_save_tool: String = save_tool.chars().take(10).collect();
                stx_file
                    .get_header_mut()
                    .created_by
                    .set_created_by(&truncated_save_tool)
                    .ok();
            }
            drop(app_state);

            on_refresh_save_tool_combo(app_controls_rc.clone(), app_state_rc.clone());
        }
        None => {
            // do nothing
        }
    }
}
