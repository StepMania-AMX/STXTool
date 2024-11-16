use crate::{
    on_refresh_mode_table_row, on_refresh_save_as_button, on_refresh_save_button,
    on_refresh_save_tool_combo, on_refresh_version_combo, AppControls, AppState,
};
use libamx::{LegacyMode, StxVersion};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn on_change_version_combo(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
    version: StxVersion,
) {
    let app_state = app_state_rc.borrow();
    let prev_version = app_state
        .get_step_file()
        .map(|step_file| step_file.get_version())
        .unwrap().clone();
    drop(app_state);

    let mut app_state = app_state_rc.borrow_mut();

    match app_state.get_step_file_mut() {
        Some(stx_file) => {
            if prev_version != version {
                stx_file.set_version(version).ok();
            }
            drop(app_state);

            on_refresh_version_combo(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
            );

            on_refresh_save_tool_combo(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
            );

            on_refresh_save_as_button(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
            );

            on_refresh_save_button(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
            );

            for mode in LegacyMode::iter() {
                on_refresh_mode_table_row(table_model_rc.clone(), mode);
            }
        }
        None => {
            // do nothing
        }
    }
}
