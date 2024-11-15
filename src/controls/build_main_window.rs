use crate::{build_global_section, build_stx_mode_table, build_selection_row, ErrorMessage};
use libui::controls::{LayoutStrategy, VerticalBox, Window, WindowType};
use libui::UI;
use screen_size::get_primary_screen_size;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_main_window(ui: Rc<UI>) -> Rc<RefCell<Window>> {
    let width = 560_i32;
    let height = 520_i32;
    let win = Rc::new(RefCell::new(Window::new(
        &ui,
        format!(
            "{} v{} by Aldo_MX",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
        .as_str(),
        width,
        height,
        WindowType::NoMenubar,
    )));

    let (screen_width, screen_height) =
        get_primary_screen_size().expect(ErrorMessage::PrimaryScreenSize.into());
    win.borrow_mut().set_position(
        (screen_width as i32 - width) / 2,
        (screen_height as i32 - height) / 2,
    );

    let mut layout = VerticalBox::new();
    layout.set_padded(true);

    layout.append(build_global_section(), LayoutStrategy::Compact);
    layout.append(build_selection_row(), LayoutStrategy::Compact);
    let (mode_table, _mode_table_data) = build_stx_mode_table(win.clone());
    layout.append(mode_table, LayoutStrategy::Stretchy);

    win.borrow_mut().set_child(layout);

    win.borrow_mut().on_closing(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    win
}
