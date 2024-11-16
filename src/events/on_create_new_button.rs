use crate::{on_refresh_step_file, AppControls, AppState, StepAuthoringTool};
use libamx::{StxFile, StxVersion};
use libui::controls::TableModel;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn on_create_new_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let mut new_button = app_controls.get_new_button_mut();

    new_button.on_clicked({
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        let table_model_rc = table_model_rc.clone();

        move |button| {
            let mut stx_file = StxFile::new(PathBuf::default(), StxVersion::Exceed2);

            stx_file
                .get_header_mut()
                .created_by
                .set_created_by(StepAuthoringTool::StxTool.into())
                .ok();

            app_state_rc.borrow_mut().open_file(stx_file);

            on_refresh_step_file(
                app_controls_rc.clone(),
                app_state_rc.clone(),
                table_model_rc.clone(),
            );
        }
    });
}
