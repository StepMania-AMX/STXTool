use crate::{
    on_create_export_all_button, on_create_new_button, on_create_open_button,
    on_create_preferred_format_combo, on_create_save_tool_combo,
    on_create_select_all_modes_checkbox, on_create_song_artist_input, on_create_song_title_input,
    on_create_step_artist_input, on_create_version_combo, AppControls, AppState,
};
use libui::controls::TableModel;
use libui::UI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_main_window(
    ui: Rc<UI>,
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    on_create_version_combo(
        ui.clone(),
        app_controls_rc.clone(),
        app_state_rc.clone(),
        table_model_rc.clone(),
    );
    on_create_save_tool_combo(ui.clone(), app_controls_rc.clone(), app_state_rc.clone());
    on_create_song_title_input(app_controls_rc.clone(), app_state_rc.clone());
    on_create_song_artist_input(app_controls_rc.clone(), app_state_rc.clone());
    on_create_step_artist_input(app_controls_rc.clone(), app_state_rc.clone());

    on_create_new_button(
        app_controls_rc.clone(),
        app_state_rc.clone(),
        table_model_rc.clone(),
    );
    on_create_open_button(
        app_controls_rc.clone(),
        app_state_rc.clone(),
        table_model_rc.clone(),
    );
    // TODO: Save button
    // TODO: Save As button

    on_create_select_all_modes_checkbox(
        ui.clone(),
        app_controls_rc.clone(),
        app_state_rc.clone(),
        table_model_rc.clone(),
    );
    // TODO: Set BPM or Delay button
    on_create_preferred_format_combo(ui.clone(), app_controls_rc.clone(), app_state_rc.clone());
    on_create_export_all_button(app_controls_rc.clone(), app_state_rc.clone());
}
