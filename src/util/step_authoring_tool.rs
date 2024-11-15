/// NOTE: The authoring tool field has a limit of 10 characters.
#[derive(Default, strum::Display, strum::EnumString, strum::IntoStaticStr)]
pub enum StepAuthoringTool {
    #[default]
    #[strum(serialize = "STXTool")]
    StxTool,

    #[strum(serialize = "STEPEdit")]
    StepEdit,

    StepMania,
}
