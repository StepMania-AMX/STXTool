use crate::{AppControls, DialogTitle, ErrorMessage, StxModeColumn};
use libui::controls::{SelectionMode, Table, TableModel, TableParameters, TextColumnParameters};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn build_stx_mode_table(
    app_controls_rc: Rc<RefCell<AppControls>>,
    table_model_rc: Rc<RefCell<TableModel>>,
) -> Table {
    let mut table = Table::new(TableParameters::new(table_model_rc.clone()));

    StxModeColumn::iter()
        .enumerate()
        .take(StxModeColumn::Selection as usize)
        .for_each(|(i, col)| {
            let col_index = i as i32;
            match col {
                StxModeColumn::Mode => table.append_checkbox_text_column(
                    col.into(),
                    StxModeColumn::Selection as i32,
                    Table::COLUMN_EDITABLE,
                    col_index,
                    Table::COLUMN_READONLY,
                ),
                StxModeColumn::Difficulty => {
                    table.append_button_column(col.into(), col_index, Table::COLUMN_EDITABLE)
                }
                StxModeColumn::ActionEdit | StxModeColumn::ActionDelete => {
                    table.append_button_column("", col_index, Table::COLUMN_EDITABLE);
                }
                _ => {
                    table.append_text_column_with_params(
                        col.into(),
                        col_index,
                        Table::COLUMN_READONLY,
                        TextColumnParameters {
                            text_color_column: StxModeColumn::Color as i32,
                        },
                    );
                }
            }
            table.set_column_width(col_index, col.get_column_width());
        });

    table.set_selection_mode(SelectionMode::ZeroOrOne);

    table.on_header_clicked({
        let app_controls_rc = app_controls_rc.clone();
        move |_table, _column| {
            app_controls_rc.borrow_mut().get_main_win().modal_err(
                DialogTitle::OperationNotAllowed.into(),
                ErrorMessage::SortingDisabled.into(),
            );
        }
    });

    table
}
