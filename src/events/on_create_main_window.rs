use crate::{on_create_new_button, on_create_open_button, AppControls, AppState};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_main_window(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
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
}
