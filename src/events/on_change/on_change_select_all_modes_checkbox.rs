use crate::{
    on_refresh_mode_table_row, on_refresh_select_all_modes_checkbox,
    on_refresh_set_bpm_or_delay_button, AppControls, AppState,
};
use libamx::LegacyMode;
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn on_change_select_all_modes_checkbox(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
    is_select_all: bool,
) {
    let mut app_state = app_state_rc.borrow_mut();
    let has_step_file = app_state.get_step_file_mut().is_some();
    if is_select_all {
        app_state.set_select_all();
    } else {
        app_state.set_select_none();
    }
    drop(app_state);

    on_refresh_select_all_modes_checkbox(app_controls_rc.clone(), has_step_file, is_select_all);
    on_refresh_set_bpm_or_delay_button(app_controls_rc.clone(), has_step_file, !is_select_all);
    for mode in LegacyMode::iter() {
        on_refresh_mode_table_row(table_model_rc.clone(), mode);
    }
}
