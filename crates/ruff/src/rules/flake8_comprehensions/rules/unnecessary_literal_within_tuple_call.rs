use log::error;
use rustpython_parser::ast::{Expr, ExprKind};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;
use crate::registry::{AsRule, Diagnostic};
use crate::rules::flake8_comprehensions::fixes;
use crate::violation::AlwaysAutofixableViolation;

use super::helpers;

#[violation]
pub struct UnnecessaryLiteralWithinTupleCall {
    pub literal: String,
}

impl AlwaysAutofixableViolation for UnnecessaryLiteralWithinTupleCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryLiteralWithinTupleCall { literal } = self;
        if literal == "list" {
            format!(
                "Unnecessary `{literal}` literal passed to `tuple()` (rewrite as a `tuple` \
                 literal)"
            )
        } else {
            format!(
                "Unnecessary `{literal}` literal passed to `tuple()` (remove the outer call to \
                 `tuple()`)"
            )
        }
    }

    fn autofix_title(&self) -> String {
        let UnnecessaryLiteralWithinTupleCall { literal } = self;
        {
            if literal == "list" {
                "Rewrite as a `tuple` literal".to_string()
            } else {
                "Remove outer `tuple` call".to_string()
            }
        }
    }
}

/// C409
pub fn unnecessary_literal_within_tuple_call(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
) {
    let Some(argument) = helpers::first_argument_with_matching_function("tuple", func, args) else {
        return;
    };
    if !checker.ctx.is_builtin("tuple") {
        return;
    }
    let argument_kind = match argument {
        ExprKind::Tuple { .. } => "tuple",
        ExprKind::List { .. } => "list",
        _ => return,
    };
    let mut diagnostic = Diagnostic::new(
        UnnecessaryLiteralWithinTupleCall {
            literal: argument_kind.to_string(),
        },
        Range::from(expr),
    );
    if checker.patch(diagnostic.kind.rule()) {
        match fixes::fix_unnecessary_literal_within_tuple_call(
            checker.locator,
            checker.stylist,
            expr,
        ) {
            Ok(fix) => {
                diagnostic.amend(fix);
            }
            Err(e) => error!("Failed to generate fix: {e}"),
        }
    }
    checker.diagnostics.push(diagnostic);
}
