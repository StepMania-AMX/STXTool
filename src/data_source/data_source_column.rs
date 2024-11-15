#[derive(
    Clone,
    Copy,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::EnumString,
    strum::FromRepr,
    strum::IntoStaticStr,
)]
#[repr(i32)]
pub enum DataSourceColumn {
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

impl DataSourceColumn {
    pub fn get_column_width(&self) -> i32 {
        use DataSourceColumn::*;
        match self {
            Mode => 50,
            Difficulty | BPM | Delay => 35,
            Stats => 45,
            ActionImport | ActionExport | ActionDelete => 60,
            Selection => 0, // not visible
        }
    }
}
