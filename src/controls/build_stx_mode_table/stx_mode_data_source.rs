use crate::{is_dark_mode, AppState, DialogTitle, ErrorMessage, StxModeColumn};
use libamx::LegacyMode;
use libui::controls::{TableDataSource, TableValue, TableValueType};
use strum::EnumCount;

impl AppState {
    const COLOR_BLACK: TableValue = TableValue::Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    const COLOR_GRAY: TableValue = TableValue::Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    const COLOR_WHITE: TableValue = TableValue::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    const UNDELETE: &'static str = "Undelete";
}

impl TableDataSource for AppState {
    fn num_columns(&mut self) -> i32 {
        StxModeColumn::COUNT as i32
    }

    fn num_rows(&mut self) -> i32 {
        LegacyMode::COUNT as i32
    }

    fn column_type(&mut self, column: i32) -> TableValueType {
        let column = StxModeColumn::from_repr(column);
        match column {
            Some(StxModeColumn::Selection) => TableValueType::Int,
            Some(StxModeColumn::Color) => TableValueType::Color,
            _ => TableValueType::String,
        }
    }

    fn cell(&mut self, column: i32, row: i32) -> TableValue {
        let column = StxModeColumn::from_repr(column);
        let mode = LegacyMode::from_repr(row as u32);

        match (column, mode) {
            (Some(StxModeColumn::Selection), Some(mode)) => {
                TableValue::Int(self.get_is_selected(mode).unwrap_or(0))
            }
            (Some(StxModeColumn::Color), Some(mode)) => {
                match (self.get_is_deleted(mode), is_dark_mode()) {
                    (true, _) => Self::COLOR_GRAY,
                    (false, true) => Self::COLOR_WHITE,
                    (false, false) => Self::COLOR_BLACK,
                }
            }
            (Some(column), Some(mode)) => {
                let value = match column {
                    StxModeColumn::Mode => Some(mode.get_id().to_string()),
                    StxModeColumn::Difficulty => self.get_difficulty(mode),
                    StxModeColumn::BPM => self.get_bpm(mode),
                    StxModeColumn::Delay => self.get_delay(mode),
                    StxModeColumn::Splits => self.get_splits(mode),
                    StxModeColumn::ActionEdit => Some(column.to_string()),
                    StxModeColumn::ActionDelete => {
                        if self.get_is_deleted(mode) {
                            Some(Self::UNDELETE.to_string())
                        } else {
                            Some(column.to_string())
                        }
                    }
                    StxModeColumn::Selection | StxModeColumn::Color => None, // should never happen
                };
                TableValue::String(value.unwrap_or(Self::PLACEHOLDER.to_string()))
            }
            _ => TableValue::String(Self::PLACEHOLDER.to_string()),
        }
    }

    fn set_cell(&mut self, column: i32, row: i32, value: TableValue) {
        if !self.is_enabled() {
            let app_controls_rc = self.get_app_controls_rc();
            let mut app_controls = app_controls_rc.borrow_mut();
            app_controls.get_main_win().modal_err(
                DialogTitle::OperationNotAllowed.into(),
                ErrorMessage::StepFileNotOpen.into(),
            );
            return;
        }

        let column = StxModeColumn::from_repr(column);
        let mode = LegacyMode::from_repr(row as u32);
        match (column, mode) {
            (Some(StxModeColumn::ActionEdit), Some(mode)) => self.set_next_edit(mode),
            (Some(StxModeColumn::ActionDelete), Some(mode)) => self.set_next_delete(mode),
            (Some(StxModeColumn::Selection), Some(mode)) => self.toggle_is_selected(mode),
            (Some(column), Some(mode)) => match value {
                TableValue::Int(value) => {
                    println!("Set cell ({}, {}) to int({})", column, mode, value);
                }
                TableValue::String(value) => {
                    println!("Set cell ({}, {}) to string({})", column, mode, value);
                }
                TableValue::Color { r, g, b, a } => {
                    println!(
                        "Set cell ({}, {}) to rgba({}, {}, {}, {})",
                        column, mode, r, g, b, a
                    );
                }
            },
            _ => {}
        }

        // TODO: Remove debug logging.
        println!("{:?}", self);
    }
}
