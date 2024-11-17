use crate::{on_change_preferred_format_combo, AppControls, AppState, StepFormat};
use libui::UI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_preferred_format_combo(
    ui: Rc<UI>,
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let preferred_format_combo = app_controls.get_preferred_format_combo();
    for format in StepFormat::EXPORT_FORMAT {
        preferred_format_combo.append(format.into());
    }
    preferred_format_combo.set_selected(StepFormat::EXPORT_DEFAULT_INDEX as i32);
    preferred_format_combo.disable();

    preferred_format_combo.on_selected(&ui, {
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        move |index| {
            on_change_preferred_format_combo(app_controls_rc.clone(), app_state_rc.clone(), index);
        }
    });
}
