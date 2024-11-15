use libui::controls::Window;
use std::cell::OnceCell;

#[derive(Default)]
pub struct AppControls {
    main_win: OnceCell<Window>,
}

impl AppControls {
    pub(crate) fn get_main_win(&self) -> &Window {
        self.main_win.get().unwrap()
    }

    pub(crate) fn get_main_win_mut(&mut self) -> &mut Window {
        self.main_win.get_mut().unwrap()
    }

    pub(crate) fn set_main_win(&mut self, win: Window) {
        self.main_win.take();
        self.main_win.set(win).unwrap_or_else(|_| unreachable!());
    }
}
