#![allow(dead_code, unused_imports, unused_variables)]

use once_cell::sync::Lazy;
use regex::Regex;

use ruff_macros::{derive_message_formats, violation};

use crate::registry::DiagnosticKind;
use crate::violation::Violation;

#[violation]
pub struct TabBeforeOperator;

impl Violation for TabBeforeOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Tab before operator")
    }
}

#[violation]
pub struct MultipleSpacesBeforeOperator;

impl Violation for MultipleSpacesBeforeOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Multiple spaces before operator")
    }
}

#[violation]
pub struct TabAfterOperator;

impl Violation for TabAfterOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Tab after operator")
    }
}

#[violation]
pub struct MultipleSpacesAfterOperator;

impl Violation for MultipleSpacesAfterOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Multiple spaces after operator")
    }
}

static OPERATOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^,\s](\s*)(?:[-+*/|!<=>%&^]+|:=)(\s*)").unwrap());

/// E221, E222, E223, E224
#[cfg(feature = "logical_lines")]
pub fn space_around_operator(line: &str) -> Vec<(usize, DiagnosticKind)> {
    let mut diagnostics = vec![];
    for line_match in OPERATOR_REGEX.captures_iter(line) {
        let before = line_match.get(1).unwrap();
        let after = line_match.get(2).unwrap();

        if before.as_str().contains('\t') {
            diagnostics.push((before.start(), TabBeforeOperator.into()));
        } else if before.as_str().len() > 1 {
            diagnostics.push((before.start(), MultipleSpacesBeforeOperator.into()));
        }

        if after.as_str().contains('\t') {
            diagnostics.push((after.start(), TabAfterOperator.into()));
        } else if after.as_str().len() > 1 {
            diagnostics.push((after.start(), MultipleSpacesAfterOperator.into()));
        }
    }
    diagnostics
}

#[cfg(not(feature = "logical_lines"))]
pub fn space_around_operator(_line: &str) -> Vec<(usize, DiagnosticKind)> {
    vec![]
}
