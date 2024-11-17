use crate::{on_change_select_all_modes_checkbox, AppControls, AppState};
use libui::controls::TableModel;
use libui::UI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_select_all_modes_checkbox(
    ui: Rc<UI>,
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let select_all_modes_checkbox = app_controls.get_select_all_modes_checkbox_mut();

    select_all_modes_checkbox.on_toggled(&ui, {
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        let table_model_rc = table_model_rc.clone();
        move |checkbox_value| {
            let should_honor_checkbox_value = {
                let app_state = app_state_rc.borrow();
                checkbox_value && app_state.get_is_selected_none().unwrap_or(false)
                    || !checkbox_value && app_state.get_is_selected_all().unwrap_or(false)
            };
            on_change_select_all_modes_checkbox(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
                checkbox_value && should_honor_checkbox_value,
            )
        }
    });
}