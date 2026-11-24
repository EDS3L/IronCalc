#![allow(clippy::unwrap_used)]

//! Defined names can hold a constant or any formula, and names that point to
//! cells move with them when rows are inserted.

use crate::test::util::new_empty_model;

#[test]
fn named_constant() {
    let mut model = new_empty_model();
    model.new_defined_name("VAT", None, "0.23").unwrap();
    model._set("A1", "100");
    model._set("B1", "=A1*VAT");
    model.evaluate();
    assert_eq!(model._get_text("B1"), "23");
}

#[test]
fn named_formula() {
    let mut model = new_empty_model();
    model._set("A1", "21");
    model.new_defined_name("Double", None, "Sheet1!$A$1*2").unwrap();
    model._set("B1", "=Double");
    model.evaluate();
    assert_eq!(model._get_text("B1"), "42");
}

#[test]
fn name_moves_with_inserted_rows() {
    let mut model = new_empty_model();
    model._set("A5", "7");
    model.new_defined_name("Total", None, "Sheet1!$A$5").unwrap();
    model._set("B1", "=Total");
    model.evaluate();
    assert_eq!(model._get_text("B1"), "7");
    model.insert_rows(0, 3, 2).unwrap();
    model.evaluate();
    assert_eq!(model._get_text("A7"), "7");
    assert_eq!(model._get_text("B1"), "7");
    assert_eq!(model.workbook.defined_names[0].formula, "Sheet1!$A$7");
}

#[test]
fn name_moves_with_deleted_rows() {
    let mut model = new_empty_model();
    model._set("A5", "7");
    model.new_defined_name("Total", None, "Sheet1!$A$5").unwrap();
    model._set("B1", "=Total");
    model.delete_rows(0, 2, 1).unwrap();
    model.evaluate();
    assert_eq!(model._get_text("B1"), "7");
    assert_eq!(model.workbook.defined_names[0].formula, "Sheet1!$A$4");
}
