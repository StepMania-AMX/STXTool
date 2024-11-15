use libamx::StxVersion;
use libui::controls::Group;

pub fn build_global_section() -> Group {
    libui::layout! { ui,
        let group = Group("") {
            let _cnpgcopy = LayoutGrid(padded: true) {
                (0, 0)(1, 1) Neither (Fill, Fill): let _u3tqfyyx = HorizontalBox(padded: false) {
                    Stretchy: let _a7sttrqg = Spacer()
                    Compact: let version_label = Label("Version")
                }
                (1, 0)(1, 1) Neither (Fill, Fill): let version_combo = Combobox() {
                    &StxVersion::Rebirth.to_string(),
                    &StxVersion::Exceed2.to_string(),
                    &StxVersion::NewXenesis.to_string()
                }

                (2, 0)(1, 1) Neither (Fill, Fill): let save_tool_label = Label("Save Tool")
                (3, 0)(1, 1) Neither (Fill, Fill): let save_tool_combo = EditableCombobox() {
                    "STXTool", "STEPEdit"
                }

                (0, 1)(1, 1) Neither (Fill, Fill): let _kfgf7kyh = HorizontalBox(padded: false) {
                    Stretchy: let _v4qsk1ab = Spacer()
                    Compact: let song_title_label = Label("Song Title")
                }
                (1, 1)(3, 1) Neither (Fill, Fill): let song_title_input = Entry()

                (0, 2)(1, 1) Neither (Fill, Fill): let _hmgni2ky = HorizontalBox(padded: false) {
                    Stretchy: let _q8w7nkfq = Spacer()
                    Compact: let song_artist_label = Label("Song Artist")
                }
                (1, 2)(3, 1) Neither (Fill, Fill): let song_artist_input = Entry()

                (0, 3)(1, 1) Neither (Fill, Fill): let _tgsred2f = HorizontalBox(padded: false) {
                    Stretchy: let _fc3rn3wo = Spacer()
                    Compact: let step_artist_label = Label("Step Artist")
                }
                (1, 3)(3, 1) Neither (Fill, Fill): let step_artist_input = Entry()

                (4, 0)(2, 2) Neither (Fill, Fill): let open_button = Button("Open")
                (4, 2)(2, 1) Neither (Fill, Fill): let save_button = Button("Save")
                (4, 3)(2, 1) Neither (Fill, Fill): let save_as_button = Button("Save As…")
                (6, 0)(1, 4) Neither (Fill, Fill): let _ygsh4d8z = Spacer()
                (7, 0)(1, 4) Neither (Fill, Fill): let _fdadjst2 = Spacer()
            }
        }
    }

    group
}
