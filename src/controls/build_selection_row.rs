#![allow(unused_mut)]

use crate::{AppControls, AppState, StepFormat};
use libui::controls::{Button, Checkbox, Combobox, HorizontalBox, LayoutStrategy, Spacer};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn build_selection_row(
    _app_controls_rc: Rc<RefCell<AppControls>>,
    _app_state_rc: Rc<RefCell<AppState>>,
) -> HorizontalBox {
    let mut selection_row = HorizontalBox::new();
    selection_row.set_padded(true);

    let mut select_all_checkbox = Checkbox::new("");
    selection_row.append(select_all_checkbox, LayoutStrategy::Compact);

    let mut set_timing_button = Button::new("Set BPM or Delay");
    selection_row.append(set_timing_button, LayoutStrategy::Compact);

    selection_row.append(Spacer::new(), LayoutStrategy::Stretchy);

    let mut preferred_format_combo = Combobox::new();
    for format in StepFormat::iter() {
        preferred_format_combo.append(format.into());
    }
    preferred_format_combo.set_selected(StepFormat::Ucs as i32);
    selection_row.append(preferred_format_combo, LayoutStrategy::Compact);

    let mut export_all_button = Button::new("Export All");
    selection_row.append(export_all_button, LayoutStrategy::Compact);

    selection_row
}
