use rustpython_parser::ast::{ArgData, Expr, ExprKind, Stmt, StmtKind};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::{Range, ScopeKind};

use crate::checkers::ast::Checker;
use crate::registry::{AsRule, Diagnostic};
use crate::rules::pyupgrade::fixes;
use crate::violation::AlwaysAutofixableViolation;

#[violation]
pub struct SuperCallWithParameters;

impl AlwaysAutofixableViolation for SuperCallWithParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use `super()` instead of `super(__class__, self)`")
    }

    fn autofix_title(&self) -> String {
        "Remove `__super__` parameters".to_string()
    }
}

/// Returns `true` if a call is an argumented `super` invocation.
fn is_super_call_with_arguments(func: &Expr, args: &[Expr]) -> bool {
    if let ExprKind::Name { id, .. } = &func.node {
        id == "super" && !args.is_empty()
    } else {
        false
    }
}

/// UP008
pub fn super_call_with_parameters(checker: &mut Checker, expr: &Expr, func: &Expr, args: &[Expr]) {
    // Only bother going through the super check at all if we're in a `super` call.
    // (We check this in `super_args` too, so this is just an optimization.)
    if !is_super_call_with_arguments(func, args) {
        return;
    }
    let scope = checker.ctx.current_scope();
    let parents: Vec<&Stmt> = checker.ctx.parents.iter().map(Into::into).collect();

    // Check: are we in a Function scope?
    if !matches!(scope.kind, ScopeKind::Function { .. }) {
        return;
    }

    let mut parents = parents.iter().rev();

    // For a `super` invocation to be unnecessary, the first argument needs to match
    // the enclosing class, and the second argument needs to match the first
    // argument to the enclosing function.
    let [first_arg, second_arg] = args else {
        return;
    };

    // Find the enclosing function definition (if any).
    let Some(StmtKind::FunctionDef {
        args: parent_args, ..
    }) = parents
        .find(|stmt| matches!(stmt.node, StmtKind::FunctionDef { .. }))
        .map(|stmt| &stmt.node) else {
        return;
    };

    // Extract the name of the first argument to the enclosing function.
    let Some(ArgData {
        arg: parent_arg, ..
    }) = parent_args.args.first().map(|expr| &expr.node) else {
        return;
    };

    // Find the enclosing class definition (if any).
    let Some(StmtKind::ClassDef {
        name: parent_name, ..
    }) = parents
        .find(|stmt| matches!(stmt.node, StmtKind::ClassDef { .. }))
        .map(|stmt| &stmt.node) else {
        return;
    };

    let (
        ExprKind::Name {
            id: first_arg_id, ..
        },
        ExprKind::Name {
            id: second_arg_id, ..
        },
    ) = (&first_arg.node, &second_arg.node) else {
        return;
    };

    if !(first_arg_id == parent_name && second_arg_id == parent_arg) {
        return;
    }

    let mut diagnostic = Diagnostic::new(SuperCallWithParameters, Range::from(expr));
    if checker.patch(diagnostic.kind.rule()) {
        if let Some(fix) = fixes::remove_super_arguments(checker.locator, checker.stylist, expr) {
            diagnostic.amend(fix);
        }
    }
    checker.diagnostics.push(diagnostic);
}
