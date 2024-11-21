use crate::{AppControls, AppState, StepFormat};
use libamx::LegacyMode;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn on_create_export_all_button(
    app_controls_rc: Rc<RefCell<AppControls>>,
    app_state_rc: Rc<RefCell<AppState>>,
) {
    let mut app_controls = app_controls_rc.borrow_mut();
    let export_all_button = app_controls.get_export_all_button();

    export_all_button.on_clicked({
        let app_controls_rc = app_controls_rc.clone();
        let app_state_rc = app_state_rc.clone();

        move |_export_all_button| {
            let app_state = app_state_rc.borrow();
            if let (Some(stx_file), Some(preferred_format_index)) = (
                app_state.get_step_file(),
                app_state.get_preferred_format_index(),
            ) {
                let mut app_controls = app_controls_rc.borrow_mut();
                if let (Some(selected_folder), Some(preferred_format)) = (
                    app_controls.get_main_win().open_folder(),
                    StepFormat::EXPORT_FORMAT.get(preferred_format_index as usize),
                ) {
                    let export_all_folder = selected_folder
                        .join(stx_file.get_path().file_stem().expect("Path buffer error."));
                    fs::create_dir_all(&export_all_folder).expect(&format!(
                        "Error creating output directory: {}",
                        export_all_folder.display()
                    ));
                    for mode in
                        LegacyMode::iter().filter(|mode| preferred_format.is_mode_compatible(mode))
                    {
                        let step_data = stx_file
                            .parse_step_data(mode)
                            .expect(&format!("Error loading step data for mode: {}", mode));
                        if step_data.is_empty() {
                            continue;
                        }
                        preferred_format
                            .save_stx_step_data(&step_data, &export_all_folder)
                            .expect(&format!(
                                "Error saving {} file for mode: {}",
                                preferred_format, mode
                            ))
                    }
                }
            }
        }
    });
}
