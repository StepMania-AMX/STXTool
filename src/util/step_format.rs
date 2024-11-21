#![allow(unreachable_patterns)]

use libamx::{LegacyMode, StxStepData, UcsFile, UcsFormat, UcsStepData};
use std::path::PathBuf;

#[derive(
    Clone,
    Copy,
    Debug,
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
    #[strum(serialize = "STF (1st)")]
    Stf1024 = 0,

    #[strum(serialize = "STF (2nd)")]
    Stf2048 = 1,

    #[strum(serialize = "ST2")]
    St2 = 2,

    #[strum(serialize = "NOT4")]
    Not4 = 3,

    #[strum(serialize = "NOT5")]
    Not5 = 4,

    #[strum(serialize = "KSF (KIU)")]
    KsfKiu = 5,

    #[strum(serialize = "SM")]
    Sm = 6,

    #[strum(serialize = "KSF (DM)")]
    KsfDm = 7,

    #[strum(serialize = "NX10")]
    Nx10 = 8,

    #[strum(serialize = "SMA")]
    Sma = 9,

    #[strum(serialize = "KSF (AMX)")]
    KsfAmx = 10,

    #[strum(serialize = "NX20")]
    Nx20 = 11,

    #[strum(serialize = "SSC")]
    Ssc = 12,

    #[strum(serialize = "UCS")]
    Ucs = 13,

    #[strum(serialize = "SSC (Infinity)")]
    SscInfinity = 14,

    #[strum(serialize = "UCS (AMX)")]
    UcsAmx = 15,

    #[strum(serialize = "SSC (StepF2)")]
    SscStepF2 = 16,
}

impl StepFormat {
    pub const EXPORT_DEFAULT: StepFormat = StepFormat::Ucs;

    pub const EXPORT_DEFAULT_INDEX: usize = 6;

    pub const EXPORT_FORMAT: [StepFormat; 8] = [
        StepFormat::Stf1024,
        StepFormat::Stf2048,
        StepFormat::KsfDm,
        StepFormat::Nx10,
        StepFormat::Nx20,
        StepFormat::KsfAmx,
        StepFormat::Ucs,
        StepFormat::UcsAmx,
    ];

    pub fn add_extension(&self, filename: &str) -> String {
        let extension = match self {
            StepFormat::Stf1024 => "STF",
            StepFormat::Stf2048 => "STF",
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
            StepFormat::Stf1024
            | StepFormat::Stf2048
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

    pub fn init() {
        assert_eq!(
            Self::EXPORT_FORMAT[Self::EXPORT_DEFAULT_INDEX],
            Self::EXPORT_DEFAULT,
            "EXPORT_DEFAULT_INDEX mismatch."
        );
    }

    pub fn is_mode_compatible(&self, mode: &LegacyMode) -> bool {
        match (self, mode) {
            (StepFormat::Nx10 | StepFormat::Nx20 | StepFormat::UcsAmx, LegacyMode::LightMap) => {
                true
            }
            (_, LegacyMode::LightMap) => false,
            _ => true,
        }
    }

    pub fn save_stx_step_data(
        &self,
        stx_step_data: &StxStepData,
        out_folder: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            StepFormat::Ucs => {
                let ucs_format = UcsFormat::V1;
                let ucs_step_data = UcsStepData::from_stx_step_data(&stx_step_data, ucs_format)?;
                let ucs_file = UcsFile::new(ucs_format, ucs_step_data);
                let filename = self.add_extension(stx_step_data.mode.get_id());
                let out_path = out_folder.join(filename);
                ucs_file.to_file(out_path.to_str().ok_or("Invalid path")?)?;
                Ok(())
            }
            _ => Err(format!("Unsupported step format: {}", self).into()),
        }
    }
}
