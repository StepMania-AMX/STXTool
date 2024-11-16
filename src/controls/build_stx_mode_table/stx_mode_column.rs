#[derive(
    Clone,
    Copy,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::FromRepr,
    strum::IntoStaticStr,
)]
#[repr(i32)]
pub enum StxModeColumn {
    Mode = 0,
    Difficulty = 1,
    BPM = 2,
    Delay = 3,
    Stats = 4,

    #[strum(serialize = "Import")]
    ActionImport = 5,

    #[strum(serialize = "Export")]
    ActionExport = 6,

    #[strum(serialize = "Delete")]
    ActionDelete = 7,

    Selection = 8,
}

impl StxModeColumn {
    pub fn get_column_width(&self) -> i32 {
        use StxModeColumn::*;
        match self {
            Mode => 45,
            Difficulty => 40,
            BPM | Delay | Stats => 55,
            ActionImport | ActionExport | ActionDelete => 40,
            Selection => 0, // not visible
        }
    }
}
