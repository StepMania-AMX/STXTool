use crate::{AppControls, AppState};
use libui::controls::TextEntry;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_song_title_input(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let song_title_input = app_controls.get_song_title_input();

    let app_state = app_state_rc.borrow();
    if let Some(stx_file) = app_state.get_step_file() {
        song_title_input.enable();
        song_title_input.set_value(stx_file.get_header().song_title.as_str());
    } else {
        song_title_input.disable();
        song_title_input.set_value("");
    }
}
