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
pub struct UnnecessaryLiteralWithinListCall {
    pub literal: String,
}

impl AlwaysAutofixableViolation for UnnecessaryLiteralWithinListCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryLiteralWithinListCall { literal } = self;
        if literal == "list" {
            format!(
                "Unnecessary `{literal}` literal passed to `list()` (remove the outer call to \
                 `list()`)"
            )
        } else {
            format!(
                "Unnecessary `{literal}` literal passed to `list()` (rewrite as a `list` literal)"
            )
        }
    }

    fn autofix_title(&self) -> String {
        let UnnecessaryLiteralWithinListCall { literal } = self;
        {
            if literal == "list" {
                "Remove outer `list` call".to_string()
            } else {
                "Rewrite as a `list` literal".to_string()
            }
        }
    }
}

/// C410
pub fn unnecessary_literal_within_list_call(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
) {
    let Some(argument) = helpers::first_argument_with_matching_function("list", func, args) else {
        return;
    };
    if !checker.ctx.is_builtin("list") {
        return;
    }
    let argument_kind = match argument {
        ExprKind::Tuple { .. } => "tuple",
        ExprKind::List { .. } => "list",
        _ => return,
    };
    let mut diagnostic = Diagnostic::new(
        UnnecessaryLiteralWithinListCall {
            literal: argument_kind.to_string(),
        },
        Range::from(expr),
    );
    if checker.patch(diagnostic.kind.rule()) {
        match fixes::fix_unnecessary_literal_within_list_call(
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
