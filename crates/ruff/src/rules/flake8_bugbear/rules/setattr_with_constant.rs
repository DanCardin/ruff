use rustpython_parser::ast::{Constant, Expr, ExprContext, ExprKind, Location, Stmt, StmtKind};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::unparse_stmt;
use ruff_python_ast::source_code::Stylist;
use ruff_python_ast::types::Range;
use ruff_python_stdlib::identifiers::{is_identifier, is_mangled_private};
use ruff_python_stdlib::keyword::KWLIST;

use crate::checkers::ast::Checker;
use crate::fix::Fix;
use crate::registry::{AsRule, Diagnostic};
use crate::violation::AlwaysAutofixableViolation;

#[violation]
pub struct SetAttrWithConstant;

impl AlwaysAutofixableViolation for SetAttrWithConstant {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Do not call `setattr` with a constant attribute value. It is not any safer than \
             normal property access."
        )
    }

    fn autofix_title(&self) -> String {
        "Replace `setattr` with assignment".to_string()
    }
}

fn assignment(obj: &Expr, name: &str, value: &Expr, stylist: &Stylist) -> String {
    let stmt = Stmt::new(
        Location::default(),
        Location::default(),
        StmtKind::Assign {
            targets: vec![Expr::new(
                Location::default(),
                Location::default(),
                ExprKind::Attribute {
                    value: Box::new(obj.clone()),
                    attr: name.to_string(),
                    ctx: ExprContext::Store,
                },
            )],
            value: Box::new(value.clone()),
            type_comment: None,
        },
    );
    unparse_stmt(&stmt, stylist)
}

/// B010
pub fn setattr_with_constant(checker: &mut Checker, expr: &Expr, func: &Expr, args: &[Expr]) {
    let ExprKind::Name { id, .. } = &func.node else {
        return;
    };
    if id != "setattr" {
        return;
    }
    let [obj, name, value] = args else {
        return;
    };
    let ExprKind::Constant {
        value: Constant::Str(name),
        ..
    } = &name.node else {
        return;
    };
    if !is_identifier(name) {
        return;
    }
    if KWLIST.contains(&name.as_str()) || is_mangled_private(name.as_str()) {
        return;
    }
    // We can only replace a `setattr` call (which is an `Expr`) with an assignment
    // (which is a `Stmt`) if the `Expr` is already being used as a `Stmt`
    // (i.e., it's directly within an `StmtKind::Expr`).
    if let StmtKind::Expr { value: child } = &checker.ctx.current_stmt().node {
        if expr == child.as_ref() {
            let mut diagnostic = Diagnostic::new(SetAttrWithConstant, Range::from(expr));

            if checker.patch(diagnostic.kind.rule()) {
                diagnostic.amend(Fix::replacement(
                    assignment(obj, name, value, checker.stylist),
                    expr.location,
                    expr.end_location.unwrap(),
                ));
            }
            checker.diagnostics.push(diagnostic);
        }
    }
}
