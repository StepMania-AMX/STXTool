#![allow(unused_mut)]

use crate::AppControls;
use libui::controls::HorizontalBox;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_selection_row(app_controls_rc: Rc<RefCell<AppControls>>) -> HorizontalBox {
    libui::layout! { ui,
        let selection_row = HorizontalBox(padded: true) {
            Compact: let select_all_modes_checkbox = Checkbox("")
            Compact: let set_bpm_or_delay_button = Button("Set BPM or Delay")
            Stretchy: let _9i8x0snq = Spacer()
            Compact: let preferred_format_combo = Combobox() {
                // See `on_create_preferred_format_combo` for entries.
            }
            Compact: let export_all_button = Button("Export All")
        }
    }

    select_all_modes_checkbox.disable();
    set_bpm_or_delay_button.disable();
    preferred_format_combo.disable();
    export_all_button.disable();

    let mut app_controls = app_controls_rc.borrow_mut();
    app_controls.set_select_all_modes_checkbox(select_all_modes_checkbox);
    app_controls.set_set_bpm_or_delay_button(set_bpm_or_delay_button);
    app_controls.set_preferred_format_combo(preferred_format_combo);
    app_controls.set_export_all_button(export_all_button);

    selection_row
}
