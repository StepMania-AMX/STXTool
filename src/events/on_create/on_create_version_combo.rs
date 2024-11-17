use crate::{on_change_version_combo, AppControls, AppState, DialogTitle, ErrorMessage};
use libamx::StxVersion;
use libui::controls::TableModel;
use libui::UI;
use std::cell::RefCell;
use std::rc::Rc;

pub fn on_create_version_combo(
    ui: Rc<UI>,
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let version_combo = app_controls.get_version_combo();

    version_combo.on_selected(&ui, {
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();
        let table_model_rc = table_model_rc.clone();
        move |combo_option| {
            match StxVersion::from_repr(combo_option as u32) {
                Some(mut version) => {
                    if version == StxVersion::NewXenesis {
                        let mut app_controls = app_controls_rc.borrow_mut();
                        app_controls.get_main_win().modal_err(
                            DialogTitle::OperationNotAllowed.into(),
                            ErrorMessage::SeeEncryptionNotImplemented.into(),
                        );
                        version = StxVersion::Exceed2;
                    }
                    on_change_version_combo(
                        app_controls_rc.clone(),
                        app_state_rc.clone(),
                        table_model_rc.clone(),
                        version,
                    );
                }
                None => {
                    // do nothing
                }
            }
        }
    })
}
