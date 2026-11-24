#![allow(clippy::unwrap_used)]

//! Defined names move with the cells they point to when rows or columns are
//! inserted or deleted, and undo gives them back.

use crate::model::DefinedNameEntry;
use crate::test::user_model::util::new_empty_user_model;

fn formula_of(model: &crate::UserModel<'_>, name: &str) -> String {
    model
        .get_defined_name_list()
        .into_iter()
        .find(|entry| entry.name == name)
        .unwrap()
        .formula
}

#[test]
fn name_moves_down_with_inserted_rows() {
    let mut model = new_empty_user_model();
    model.set_user_input(0, 5, 1, "7").unwrap();
    model
        .new_defined_name(DefinedNameEntry::new("Total", "Sheet1!$A$5"))
        .unwrap();
    model.set_user_input(0, 1, 2, "=Total").unwrap();
    model.insert_rows(0, 3, 2).unwrap();
    assert_eq!(formula_of(&model, "Total"), "Sheet1!$A$7");
    assert_eq!(model.get_formatted_cell_value(0, 1, 2), Ok("7".to_string()));
    model.undo().unwrap();
    assert_eq!(formula_of(&model, "Total"), "Sheet1!$A$5");
}

#[test]
fn range_grows_when_rows_are_inserted_inside() {
    let mut model = new_empty_user_model();
    model
        .new_defined_name(DefinedNameEntry::new("Data", "Sheet1!$A$2:$A$10"))
        .unwrap();
    model.insert_rows(0, 5, 3).unwrap();
    assert_eq!(formula_of(&model, "Data"), "Sheet1!$A$2:$A$13");
}

#[test]
fn name_moves_up_with_deleted_rows_and_undo_restores_it() {
    let mut model = new_empty_user_model();
    model.set_user_input(0, 5, 1, "7").unwrap();
    model
        .new_defined_name(DefinedNameEntry::new("Total", "Sheet1!$A$5"))
        .unwrap();
    model.delete_rows(0, 2, 1).unwrap();
    assert_eq!(formula_of(&model, "Total"), "Sheet1!$A$4");
    model.undo().unwrap();
    assert_eq!(formula_of(&model, "Total"), "Sheet1!$A$5");
}

#[test]
fn name_reaching_into_deleted_rows_is_left_as_it_is() {
    let mut model = new_empty_user_model();
    model
        .new_defined_name(DefinedNameEntry::new("Data", "Sheet1!$A$2:$A$10"))
        .unwrap();
    model.delete_rows(0, 2, 1).unwrap();
    assert_eq!(formula_of(&model, "Data"), "Sheet1!$A$2:$A$10");
}

#[test]
fn name_moves_right_with_inserted_columns() {
    let mut model = new_empty_user_model();
    model
        .new_defined_name(DefinedNameEntry::new("Rate", "Sheet1!$C$1"))
        .unwrap();
    model.insert_columns(0, 2, 1).unwrap();
    assert_eq!(formula_of(&model, "Rate"), "Sheet1!$D$1");
}
