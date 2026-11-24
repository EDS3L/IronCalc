#![allow(clippy::panic)]

//! Formulas written back to text keep the parentheses their meaning needs.
//! Without them `"its "&(2<3)&" blue"` came back as `"its "&2<3&" blue"`,
//! which is a comparison of two strings and evaluates to FALSE.

use std::collections::HashMap;

use crate::expressions::types::CellReferenceRC;

use crate::expressions::parser::tests::utils::{new_parser, to_english_localized_string};

fn round_trip(formula: &str) -> String {
    let cell_reference = CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 1,
        column: 1,
    };
    let mut parser = new_parser(vec!["Sheet1".to_string()], vec![], HashMap::new());
    let parsed = parser.parse(formula, &cell_reference);
    let text = to_english_localized_string(&parsed, &cell_reference);
    let reparsed = parser.parse(&text, &cell_reference);
    let again = to_english_localized_string(&reparsed, &cell_reference);
    assert_eq!(text, again, "not stable for '{formula}'");
    text
}

#[test]
fn comparison_inside_concatenation() {
    assert_eq!(round_trip(r#""its "&(2<3)&" blue""#), r#""its "&(2<3)&" blue""#);
    assert_eq!(round_trip(r#"(1<2)&"x""#), r#"(1<2)&"x""#);
}

#[test]
fn concatenation_inside_arithmetic() {
    assert_eq!(round_trip(r#"1+("a"&"b")"#), r#"1+("a"&"b")"#);
    assert_eq!(round_trip(r#"("a"&"b")*2"#), r#"("a"&"b")*2"#);
    assert_eq!(round_trip(r#"2*("a"&"b")"#), r#"2*("a"&"b")"#);
}

#[test]
fn comparison_on_the_right_of_a_comparison() {
    assert_eq!(round_trip("1=(2<3)"), "1=(2<3)");
}

#[test]
fn no_extra_parentheses() {
    assert_eq!(round_trip(r#""a"&"b"&"c""#), r#""a"&"b"&"c""#);
    assert_eq!(round_trip("1<2"), "1<2");
    assert_eq!(round_trip(r#"1+2&"x""#), r#"1+2&"x""#);
}
