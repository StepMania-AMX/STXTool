use crate::{on_refresh_step_file, AppControls, AppState, ErrorMessage};
use libamx::StxFile;
use libui::controls::TableModel;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_open_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let open_button = app_controls.get_open_button_mut();

    open_button.on_clicked({
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        let table_model_rc = table_model_rc.clone();

        move |_open_button| {
            let app_controls = app_controls_rc.borrow();

            if let Some(stx_path) = app_controls.get_main_win().open_file() {
                drop(app_controls);

                let stx_file =
                    StxFile::from_file(stx_path).expect(ErrorMessage::StepFileRead.into());

                app_state_rc.borrow_mut().open_file(stx_file);

                on_refresh_step_file(
                    app_controls_rc.clone(),
                    app_state_rc.clone(),
                    table_model_rc.clone(),
                );
            }
        }
    });
}
