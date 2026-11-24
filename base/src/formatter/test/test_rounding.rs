#![allow(clippy::unwrap_used)]

//! Shown digits are rounded half away from zero, like Excel, on the 15
//! significant digits Excel works with.

use crate::{formatter::format::format_number, locale::get_locale};

fn show(value: f64, code: &str) -> String {
    format_number(value, code, get_locale("en").unwrap()).text
}

#[test]
fn halves_round_away_from_zero() {
    assert_eq!(show(2.5, "0"), "3");
    assert_eq!(show(3.5, "0"), "4");
    assert_eq!(show(-2.5, "0"), "-3");
    assert_eq!(show(0.125, "0.00"), "0.13");
    assert_eq!(show(1.005, "0.00"), "1.01");
    assert_eq!(show(9.995, "0.00"), "10.00");
    assert_eq!(show(1234.5, "#,##0"), "1,235");
}

#[test]
fn not_halves_unchanged() {
    assert_eq!(show(2.4999, "0"), "2");
    assert_eq!(show(0.124, "0.00"), "0.12");
    assert_eq!(show(7.0, "000"), "007");
}

#[test]
fn sign_of_values_that_round_to_non_zero() {
    assert_eq!(show(-0.5, "0"), "-1");
    assert_eq!(show(-0.005, "0.00"), "-0.01");
}
