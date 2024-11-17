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
    checkbox_value: bool,
) {
    let mut app_state = app_state_rc.borrow_mut();

    if checkbox_value {
        app_state.set_select_all();
    } else {
        app_state.set_select_none();
    }

    on_refresh_select_all_modes_checkbox(app_controls_rc.clone(), &*app_state);
    on_refresh_set_bpm_or_delay_button(app_controls_rc.clone(), &*app_state);

    drop(app_state);

    for mode in LegacyMode::iter() {
        on_refresh_mode_table_row(table_model_rc.clone(), mode);
    }
}
