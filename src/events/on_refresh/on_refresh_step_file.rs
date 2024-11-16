use crate::{
    on_refresh_export_all_button, on_refresh_mode_table_row, on_refresh_preferred_format_combo,
    on_refresh_save_as_button, on_refresh_save_button, on_refresh_save_tool_combo,
    on_refresh_select_all_modes_checkbox, on_refresh_set_bpm_or_delay_button,
    on_refresh_song_artist_input, on_refresh_song_title_input, on_refresh_step_artist_input,
    on_refresh_version_combo, AppControls, AppState,
};
use libamx::LegacyMode;
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn on_refresh_step_file(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    on_refresh_version_combo(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_save_tool_combo(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_song_title_input(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_song_artist_input(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_step_artist_input(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_save_as_button(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_save_button(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_select_all_modes_checkbox(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_set_bpm_or_delay_button(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_preferred_format_combo(app_controls_rc.clone(), app_state_rc.clone());
    on_refresh_export_all_button(app_controls_rc.clone(), app_state_rc.clone());
    for mode in LegacyMode::iter() {
        on_refresh_mode_table_row(table_model_rc.clone(), mode);
    }
}
