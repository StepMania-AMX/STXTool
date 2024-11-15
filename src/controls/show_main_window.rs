use crate::{
    build_global_section, build_selection_row, build_stx_mode_table, AppControls, AppState,
    ErrorMessage,
};
use libui::controls::{LayoutStrategy, VerticalBox, Window, WindowType};
use libui::UI;
use screen_size::get_primary_screen_size;
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_main_window(ui: Rc<UI>) {
    let app_controls_rc = Rc::new(RefCell::new(AppControls::default()));
    let app_state_rc = Rc::new(RefCell::new(AppState::new(app_controls_rc.clone())));

    let width = 560_i32;
    let height = 520_i32;
    let (screen_width, screen_height) =
        get_primary_screen_size().expect(ErrorMessage::PrimaryScreenSize.into());

    app_controls_rc.borrow_mut().set_main_win({
        let mut win = Window::new(
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
        );

        win.set_position(
            (screen_width as i32 - width) / 2,
            (screen_height as i32 - height) / 2,
        );

        win
    });

    let mut layout = VerticalBox::new();
    layout.set_padded(true);

    layout.append(
        build_global_section(app_controls_rc.clone(), app_state_rc.clone()),
        LayoutStrategy::Compact,
    );
    layout.append(
        build_selection_row(app_controls_rc.clone(), app_state_rc.clone()),
        LayoutStrategy::Compact,
    );
    layout.append(
        build_stx_mode_table(app_controls_rc.clone(), app_state_rc.clone()),
        LayoutStrategy::Stretchy,
    );

    let mut app_controls_mut = app_controls_rc.borrow_mut();
    let win = app_controls_mut.get_main_win_mut();

    win.set_child(layout);

    win.on_closing(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    win.show();
}
