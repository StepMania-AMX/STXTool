#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

mod controls;
pub use controls::*;

mod data_source;
pub use data_source::*;

mod util;
pub use util::*;

fn main() {
    let ui = std::rc::Rc::new(libui::UI::init().expect(ErrorMessage::UiLibraryInit.into()));
    let win = build_main_window(ui.clone());
    win.borrow_mut().show();
    ui.main();
}
