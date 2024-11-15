#![allow(unreachable_patterns)]

#[derive(
    Clone,
    Copy,
    Eq,
    PartialEq,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::EnumString,
    strum::FromRepr,
    strum::IntoStaticStr,
)]
#[repr(i32)]
pub enum StepFormat {
    #[strum(serialize = "STF")]
    Stf = 0,

    #[strum(serialize = "ST2")]
    St2 = 1,

    #[strum(serialize = "NOT4")]
    Not4 = 2,

    #[strum(serialize = "NOT5")]
    Not5 = 3,

    #[strum(serialize = "KSF (KIU)")]
    KsfKiu = 4,

    #[strum(serialize = "SM")]
    Sm = 5,

    #[strum(serialize = "KSF (DM)")]
    KsfDm = 6,

    #[strum(serialize = "NX10")]
    Nx10 = 7,

    #[strum(serialize = "SMA")]
    Sma = 8,

    #[strum(serialize = "KSF (AMX)")]
    KsfAmx = 9,

    #[strum(serialize = "NX20")]
    Nx20 = 10,

    #[strum(serialize = "SSC")]
    Ssc = 11,

    #[strum(serialize = "UCS")]
    Ucs = 12,

    #[strum(serialize = "SSC (Infinity)")]
    SscInfinity = 13,

    #[strum(serialize = "UCS (AMX)")]
    UcsAmx = 14,

    #[strum(serialize = "SSC (StepF2)")]
    SscStepF2 = 15,
}

impl StepFormat {
    pub fn add_extension(&self, filename: &str) -> String {
        let extension = match self {
            StepFormat::Stf => "STF",
            StepFormat::St2 => "ST2",
            StepFormat::Not4 => "NOT",
            StepFormat::Not5 => "NOT",
            StepFormat::KsfKiu => "KSF",
            StepFormat::Sm => "sm",
            StepFormat::KsfDm => "ksf",
            StepFormat::Nx10 | StepFormat::Nx20 => "NX",
            StepFormat::Sma => "sma",
            StepFormat::KsfAmx => "ksf",
            StepFormat::Ssc | StepFormat::SscInfinity | StepFormat::SscStepF2 => "ssc",
            StepFormat::Ucs | StepFormat::UcsAmx => "ucs",
        };
        match self {
            StepFormat::Stf
            | StepFormat::St2
            | StepFormat::Not4
            | StepFormat::Not5
            | StepFormat::KsfKiu
            | StepFormat::Nx10
            | StepFormat::Nx20 => format!("{}.{}", filename.to_ascii_uppercase(), extension),
            StepFormat::Sm
            | StepFormat::KsfDm
            | StepFormat::Sma
            | StepFormat::KsfAmx
            | StepFormat::Ssc
            | StepFormat::SscInfinity
            | StepFormat::SscStepF2
            | StepFormat::Ucs
            | StepFormat::UcsAmx => format!("{}.{}", filename, extension),
        }
    }
}
