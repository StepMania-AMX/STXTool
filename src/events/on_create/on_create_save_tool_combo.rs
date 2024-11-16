use crate::{on_change_save_tool_combo, AppControls, AppState};
use libui::UI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_save_tool_combo(
    ui: Rc<UI>,
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let save_tool_combo = app_controls.get_save_tool_combo_mut();

    save_tool_combo.on_changed(&ui, {
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        move |save_tool| {
            on_change_save_tool_combo(app_controls_rc.clone(), app_state_rc.clone(), save_tool)
        }
    });
}
