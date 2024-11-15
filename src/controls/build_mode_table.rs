use crate::{DataSource, DataSourceColumn};
use libui::controls::{SelectionMode, Table, TableModel, TableParameters, Window};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub fn build_mode_table(win: Rc<RefCell<Window>>) -> (Table, Rc<RefCell<DataSource>>) {
    let data = Rc::new(RefCell::new(DataSource::new(win.clone())));
    let model = Rc::new(RefCell::new(TableModel::new(data.clone())));
    let params = TableParameters::new(model.clone());
    let mut table = Table::new(params);

    DataSourceColumn::iter()
        .enumerate()
        .take(DataSourceColumn::Selection as usize)
        .for_each(|(i, col)| {
            let col_index = i as i32;
            match col {
                DataSourceColumn::Mode => table.append_checkbox_text_column(
                    col.into(),
                    DataSourceColumn::Selection as i32,
                    Table::COLUMN_EDITABLE,
                    col_index,
                    Table::COLUMN_READONLY,
                ),
                DataSourceColumn::Difficulty => {
                    table.append_button_column(col.into(), col_index, Table::COLUMN_EDITABLE)
                }
                DataSourceColumn::ActionImport
                | DataSourceColumn::ActionExport
                | DataSourceColumn::ActionDelete => {
                    table.append_button_column("", col_index, Table::COLUMN_EDITABLE);
                }
                _ => {
                    table.append_text_column(col.into(), col_index, Table::COLUMN_READONLY);
                }
            }
            table.set_column_width(col_index, col.get_column_width());
        });

    table.set_selection_mode(SelectionMode::None);

    if data.borrow().is_enabled() {
        table.enable();
    } else {
        table.disable();
    }

    table.on_header_clicked({
        let win = win.clone();
        move |_table, _column| {
            win.borrow().modal_err(
                "Operation not allowed",
                "Sorting is disabled for this table.",
            );
        }
    });

    (table, data)
}
