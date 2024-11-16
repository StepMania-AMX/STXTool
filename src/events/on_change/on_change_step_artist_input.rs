use crate::{on_refresh_step_artist_input, AppControls, AppState};
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_change_step_artist_input(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    step_artist: String,
) {
    let app_state = app_state_rc.borrow();
    let prev_step_artist = app_state
        .get_step_file()
        .map(|step_file| step_file.get_header().step_artist.clone())
        .unwrap();
    drop(app_state);

    let mut app_state = app_state_rc.borrow_mut();

    match app_state.get_step_file_mut() {
        Some(stx_file) => {
            if prev_step_artist != step_artist {
                stx_file.get_header_mut().step_artist = step_artist;
            }
            drop(app_state);

            on_refresh_step_artist_input(app_controls_rc.clone(), app_state_rc.clone());
        }
        None => {
            // do nothing
        }
    }
}
