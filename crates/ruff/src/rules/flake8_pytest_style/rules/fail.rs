use rustpython_parser::ast::{Expr, Keyword};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::SimpleCallArgs;
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

use super::helpers::{is_empty_or_null_string, is_pytest_fail};

#[violation]
pub struct FailWithoutMessage;

impl Violation for FailWithoutMessage {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("No message passed to `pytest.fail()`")
    }
}

pub fn fail_call(checker: &mut Checker, func: &Expr, args: &[Expr], keywords: &[Keyword]) {
    if is_pytest_fail(func, checker) {
        let call_args = SimpleCallArgs::new(args, keywords);
        let msg = call_args.get_argument("msg", Some(0));

        if let Some(msg) = msg {
            if is_empty_or_null_string(msg) {
                checker
                    .diagnostics
                    .push(Diagnostic::new(FailWithoutMessage, Range::from(func)));
            }
        } else {
            checker
                .diagnostics
                .push(Diagnostic::new(FailWithoutMessage, Range::from(func)));
        }
    }
}
