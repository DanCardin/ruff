use rustpython_parser::ast::{Stmt, StmtKind};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;
use ruff_python_ast::visitor::{self, Visitor};

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct RaiseWithinTry;

impl Violation for RaiseWithinTry {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Abstract `raise` to an inner function")
    }
}

#[derive(Default)]
struct RaiseStatementVisitor<'a> {
    raises: Vec<&'a Stmt>,
}

impl<'a, 'b> Visitor<'b> for RaiseStatementVisitor<'a>
where
    'b: 'a,
{
    fn visit_stmt(&mut self, stmt: &'b Stmt) {
        match stmt.node {
            StmtKind::Raise { .. } => self.raises.push(stmt),
            StmtKind::Try { .. } | StmtKind::TryStar { .. } => (),
            _ => visitor::walk_stmt(self, stmt),
        }
    }
}

/// TRY301
pub fn raise_within_try(checker: &mut Checker, body: &[Stmt]) {
    let raises = {
        let mut visitor = RaiseStatementVisitor::default();
        for stmt in body {
            visitor.visit_stmt(stmt);
        }
        visitor.raises
    };

    for stmt in raises {
        checker
            .diagnostics
            .push(Diagnostic::new(RaiseWithinTry, Range::from(stmt)));
    }
}
