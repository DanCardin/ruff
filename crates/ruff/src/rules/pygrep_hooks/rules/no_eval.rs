use rustpython_parser::ast::{Expr, ExprKind};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct NoEval;

impl Violation for NoEval {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("No builtin `eval()` allowed")
    }
}
/// PGH001 - no eval
pub fn no_eval(checker: &mut Checker, func: &Expr) {
    let ExprKind::Name { id, .. } = &func.node else {
        return;
    };
    if id != "eval" {
        return;
    }
    if !checker.ctx.is_builtin("eval") {
        return;
    }
    checker
        .diagnostics
        .push(Diagnostic::new(NoEval, Range::from(func)));
}
