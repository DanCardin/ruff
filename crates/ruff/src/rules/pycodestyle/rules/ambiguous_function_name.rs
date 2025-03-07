use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::registry::Diagnostic;
use crate::rules::pycodestyle::helpers::is_ambiguous_name;
use crate::violation::Violation;

#[violation]
pub struct AmbiguousFunctionName(pub String);

impl Violation for AmbiguousFunctionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AmbiguousFunctionName(name) = self;
        format!("Ambiguous function name: `{name}`")
    }
}

/// E743
pub fn ambiguous_function_name<F>(name: &str, locate: F) -> Option<Diagnostic>
where
    F: FnOnce() -> Range,
{
    if is_ambiguous_name(name) {
        Some(Diagnostic::new(
            AmbiguousFunctionName(name.to_string()),
            locate(),
        ))
    } else {
        None
    }
}
