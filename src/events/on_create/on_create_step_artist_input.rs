use crate::{on_change_step_artist_input, AppControls, AppState};
use libui::controls::TextEntry;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_step_artist_input(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let step_artist_input = app_controls.get_step_artist_input_mut();

    step_artist_input.on_changed({
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        move |step_artist| {
            on_change_step_artist_input(app_controls_rc.clone(), app_state_rc.clone(), step_artist)
        }
    });
}
