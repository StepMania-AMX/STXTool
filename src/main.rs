#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

mod controls;
pub use controls::*;

mod events;
pub use events::*;

mod util;
pub use util::*;

fn main() {
    on_main_start();

    let ui = std::rc::Rc::new(libui::UI::init().expect(ErrorMessage::UiLibraryInit.into()));
    show_main_window(ui.clone());
    ui.main();
}
