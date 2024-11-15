#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

mod controls;
pub use controls::*;

mod util;
pub use util::*;

fn main() {
    let ui = std::rc::Rc::new(libui::UI::init().expect(ErrorMessage::UiLibraryInit.into()));
    show_main_window(ui.clone());
    ui.main();
}
