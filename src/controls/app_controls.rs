use libui::controls::{Button, Checkbox, Combobox, EditableCombobox, Entry, Window};
use std::cell::OnceCell;

#[derive(Default)]
pub struct AppControls {
    export_all_button: OnceCell<Button>,
    main_win: OnceCell<Window>,
    new_button: OnceCell<Button>,
    open_button: OnceCell<Button>,
    preferred_format_combo: OnceCell<Combobox>,
    save_as_button: OnceCell<Button>,
    save_button: OnceCell<Button>,
    save_tool_combo: OnceCell<EditableCombobox>,
    select_all_modes_checkbox: OnceCell<Checkbox>,
    set_bpm_or_delay_button: OnceCell<Button>,
    song_artist_input: OnceCell<Entry>,
    song_title_input: OnceCell<Entry>,
    step_artist_input: OnceCell<Entry>,
    version_combo: OnceCell<Combobox>,
}

impl AppControls {
    pub(crate) fn get_export_all_button(&self) -> &Button {
        self.export_all_button.get().unwrap()
    }

    pub(crate) fn get_export_all_button_mut(&mut self) -> &mut Button {
        self.export_all_button.get_mut().unwrap()
    }

    pub(crate) fn get_main_win(&self) -> &Window {
        self.main_win.get().unwrap()
    }

    pub(crate) fn get_main_win_mut(&mut self) -> &mut Window {
        self.main_win.get_mut().unwrap()
    }

    pub(crate) fn get_new_button(&self) -> &Button {
        self.new_button.get().unwrap()
    }

    pub(crate) fn get_new_button_mut(&mut self) -> &mut Button {
        self.new_button.get_mut().unwrap()
    }

    pub(crate) fn get_open_button(&self) -> &Button {
        self.open_button.get().unwrap()
    }

    pub(crate) fn get_open_button_mut(&mut self) -> &mut Button {
        self.open_button.get_mut().unwrap()
    }

    pub(crate) fn get_preferred_format_combo(&self) -> &Combobox {
        self.preferred_format_combo.get().unwrap()
    }

    pub(crate) fn get_preferred_format_combo_mut(&mut self) -> &mut Combobox {
        self.preferred_format_combo.get_mut().unwrap()
    }

    pub(crate) fn get_save_as_button(&self) -> &Button {
        self.save_as_button.get().unwrap()
    }

    pub(crate) fn get_save_as_button_mut(&mut self) -> &mut Button {
        self.save_as_button.get_mut().unwrap()
    }

    pub(crate) fn get_save_button(&self) -> &Button {
        self.save_button.get().unwrap()
    }

    pub(crate) fn get_save_button_mut(&mut self) -> &mut Button {
        self.save_button.get_mut().unwrap()
    }

    pub(crate) fn get_save_tool_combo(&self) -> &EditableCombobox {
        self.save_tool_combo.get().unwrap()
    }

    pub(crate) fn get_save_tool_combo_mut(&mut self) -> &mut EditableCombobox {
        self.save_tool_combo.get_mut().unwrap()
    }

    pub(crate) fn get_select_all_modes_checkbox(&self) -> &Checkbox {
        self.select_all_modes_checkbox.get().unwrap()
    }

    pub(crate) fn get_select_all_modes_checkbox_mut(&mut self) -> &mut Checkbox {
        self.select_all_modes_checkbox.get_mut().unwrap()
    }

    pub(crate) fn get_set_bpm_or_delay_button(&self) -> &Button {
        self.set_bpm_or_delay_button.get().unwrap()
    }

    pub(crate) fn get_set_bpm_or_delay_button_mut(&mut self) -> &mut Button {
        self.set_bpm_or_delay_button.get_mut().unwrap()
    }

    pub(crate) fn get_song_artist_input(&self) -> &Entry {
        self.song_artist_input.get().unwrap()
    }

    pub(crate) fn get_song_artist_input_mut(&mut self) -> &mut Entry {
        self.song_artist_input.get_mut().unwrap()
    }

    pub(crate) fn get_song_title_input(&self) -> &Entry {
        self.song_title_input.get().unwrap()
    }

    pub(crate) fn get_song_title_input_mut(&mut self) -> &mut Entry {
        self.song_title_input.get_mut().unwrap()
    }

    pub(crate) fn get_step_artist_input(&self) -> &Entry {
        self.step_artist_input.get().unwrap()
    }

    pub(crate) fn get_step_artist_input_mut(&mut self) -> &mut Entry {
        self.step_artist_input.get_mut().unwrap()
    }

    pub(crate) fn get_version_combo(&self) -> &Combobox {
        self.version_combo.get().unwrap()
    }

    pub(crate) fn get_version_combo_mut(&mut self) -> &mut Combobox {
        self.version_combo.get_mut().unwrap()
    }

    pub(crate) fn set_export_all_button(&mut self, button: Button) {
        self.export_all_button.take();
        self.export_all_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_main_win(&mut self, win: Window) {
        self.main_win.take();
        self.main_win.set(win).unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_new_button(&mut self, button: Button) {
        self.new_button.take();
        self.new_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_open_button(&mut self, button: Button) {
        self.open_button.take();
        self.open_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_preferred_format_combo(&mut self, combo: Combobox) {
        self.preferred_format_combo.take();
        self.preferred_format_combo
            .set(combo)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_save_as_button(&mut self, button: Button) {
        self.save_as_button.take();
        self.save_as_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_save_button(&mut self, button: Button) {
        self.save_button.take();
        self.save_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_save_tool_combo(&mut self, combo: EditableCombobox) {
        self.save_tool_combo.take();
        self.save_tool_combo
            .set(combo)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_select_all_modes_checkbox(&mut self, checkbox: Checkbox) {
        self.select_all_modes_checkbox.take();
        self.select_all_modes_checkbox
            .set(checkbox)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_set_bpm_or_delay_button(&mut self, button: Button) {
        self.set_bpm_or_delay_button.take();
        self.set_bpm_or_delay_button
            .set(button)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_song_artist_input(&mut self, input: Entry) {
        self.song_artist_input.take();
        self.song_artist_input
            .set(input)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_song_title_input(&mut self, input: Entry) {
        self.song_title_input.take();
        self.song_title_input
            .set(input)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_step_artist_input(&mut self, input: Entry) {
        self.step_artist_input.take();
        self.step_artist_input
            .set(input)
            .unwrap_or_else(|_| unreachable!());
    }

    pub(crate) fn set_version_combo(&mut self, combo: Combobox) {
        self.version_combo.take();
        self.version_combo
            .set(combo)
            .unwrap_or_else(|_| unreachable!());
    }
}
