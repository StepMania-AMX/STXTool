use crate::{AppState, DialogTitle, ErrorMessage, StxModeColumn};
use libui::controls::{SelectionMode, SortIndicator, Table, TableModel, TableParameters, Window};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn build_stx_mode_table(win: Rc<RefCell<Window>>) -> (Table, Rc<RefCell<AppState>>) {
    let data = Rc::new(RefCell::new(AppState::new(win.clone())));
    let model = Rc::new(RefCell::new(TableModel::new(data.clone())));
    let params = TableParameters::new(model.clone());
    let mut table = Table::new(params);

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
                StxModeColumn::ActionImport
                | StxModeColumn::ActionExport
                | StxModeColumn::ActionDelete => {
                    table.append_button_column("", col_index, Table::COLUMN_EDITABLE);
                }
                _ => {
                    table.append_text_column(col.into(), col_index, Table::COLUMN_READONLY);
                }
            }
            table.set_column_width(col_index, col.get_column_width());
        });

    table.set_selection_mode(SelectionMode::ZeroOrOne);
    table.set_sort_indicator(StxModeColumn::Mode as i32, SortIndicator::Ascending);

    if data.borrow().is_enabled() {
        table.enable();
    } else {
        table.disable();
    }

    table.on_header_clicked({
        let win = win.clone();
        move |_table, _column| {
            win.borrow().modal_err(
                DialogTitle::OperationNotAllowed.into(),
                ErrorMessage::SortingDisabled.into(),
            );
        }
    });

    (table, data)
}
