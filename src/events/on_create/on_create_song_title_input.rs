use crate::{on_change_song_title_input, AppControls, AppState};
use libui::controls::TextEntry;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_song_title_input(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let song_title_input = app_controls.get_song_title_input_mut();

    song_title_input.on_changed({
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        move |song_title| {
            on_change_song_title_input(app_controls_rc.clone(), app_state_rc.clone(), song_title)
        }
    });
}
