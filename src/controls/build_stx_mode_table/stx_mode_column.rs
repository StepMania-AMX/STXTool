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
    Splits = 4,

    #[strum(serialize = "Edit")]
    ActionEdit = 5,

    #[strum(serialize = "Delete")]
    ActionDelete = 6,

    Selection = 7,
    Color = 8,
}

impl StxModeColumn {
    pub fn get_column_width(&self) -> i32 {
        use StxModeColumn::*;
        match self {
            Mode | Difficulty | BPM | Delay | Splits | ActionEdit => 55,
            ActionDelete => 65,
            Selection | Color => 0, // not visible
        }
    }
}
