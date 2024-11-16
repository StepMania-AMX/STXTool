use crate::{AppControls, AppState};
use libui::controls::{TableModel, TextEntry};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_refresh_step_artist_input(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    _table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let step_artist_input = app_controls.get_step_artist_input_mut();

    let app_state = app_state_rc.borrow();
    if let Some(stx_file) = app_state.get_step_file() {
        step_artist_input.enable();
        step_artist_input.set_value(stx_file.get_header().step_artist.as_str());
    } else {
        step_artist_input.disable();
        step_artist_input.set_value("");
    }
}
