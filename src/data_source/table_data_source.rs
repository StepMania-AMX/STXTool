use crate::{DataSource, DataSourceColumn, DialogTitle, ErrorMessage};
use libamx::LegacyMode;
use libui::controls::{TableDataSource, TableValue, TableValueType};
use strum::EnumCount;

impl TableDataSource for DataSource {
    fn num_columns(&mut self) -> i32 {
        DataSourceColumn::COUNT as i32
    }

    fn num_rows(&mut self) -> i32 {
        LegacyMode::COUNT as i32
    }

    fn column_type(&mut self, column: i32) -> TableValueType {
        let column = DataSourceColumn::from_repr(column);
        match column {
            Some(DataSourceColumn::Selection) => TableValueType::Int,
            _ => TableValueType::String,
        }
    }

    fn cell(&mut self, column: i32, row: i32) -> TableValue {
        let column = DataSourceColumn::from_repr(column);
        let mode = LegacyMode::from_repr(row as u32);

        match (column, mode) {
            (Some(DataSourceColumn::Selection), Some(mode)) => {
                TableValue::Int(self.get_is_selected(mode).unwrap_or(0))
            }
            (Some(column), Some(mode)) => {
                let value = match column {
                    DataSourceColumn::Mode => Some(mode.get_id().to_string()),
                    DataSourceColumn::Difficulty => self.get_difficulty(mode),
                    DataSourceColumn::BPM => self.get_bpm(mode),
                    DataSourceColumn::Delay => self.get_delay(mode),
                    DataSourceColumn::Stats => self.get_stats(mode),
                    DataSourceColumn::ActionImport
                    | DataSourceColumn::ActionExport
                    | DataSourceColumn::ActionDelete => Some(column.to_string()),
                    DataSourceColumn::Selection => None, // should never happen, added for completeness
                };
                TableValue::String(value.unwrap_or(Self::PLACEHOLDER.to_string()))
            }
            _ => TableValue::String(Self::PLACEHOLDER.to_string()),
        }
    }

    fn set_cell(&mut self, column: i32, row: i32, value: TableValue) {
        if !self.is_enabled() {
            self.get_active_win().borrow().modal_err(
                DialogTitle::OperationNotAllowed.into(),
                ErrorMessage::StepFileNotOpen.into(),
            );
            return;
        }

        let column = DataSourceColumn::from_repr(column);
        let mode = LegacyMode::from_repr(row as u32);
        match (column, mode) {
            (Some(DataSourceColumn::ActionImport), Some(mode)) => {
                self.set_next_import(Some(mode));
            }
            (Some(DataSourceColumn::ActionExport), Some(mode)) => {
                self.set_next_export(Some(mode));
            }
            (Some(DataSourceColumn::ActionDelete), Some(mode)) => self.set_next_delete(mode),
            (Some(DataSourceColumn::Selection), Some(mode)) => self.toggle_is_selected(mode),
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
        println!("{:?}", self);
    }
}
