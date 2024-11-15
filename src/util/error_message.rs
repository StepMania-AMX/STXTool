#[derive(strum::IntoStaticStr)]
pub enum ErrorMessage {
    #[strum(serialize = "Couldn't retrieve primary screen size.")]
    PrimaryScreenSize,

    #[strum(serialize = "Sorting is disabled for this table.")]
    SortingDisabled,

    #[strum(serialize = "Please open a step file before you do this operation.")]
    StepFileNotOpen,

    #[strum(serialize = "Couldn't initialize UI library.")]
    UiLibraryInit,
}
