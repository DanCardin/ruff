//! Generate a Markdown-compatible listing of configuration options.
use itertools::Itertools;
use ruff::settings::options::Options;
use ruff::settings::options_base::{ConfigurationOptions, OptionEntry, OptionField};

fn emit_field(output: &mut String, name: &str, field: &OptionField, group_name: Option<&str>) {
    output.push_str(&format!("#### [`{name}`](#{name})\n"));
    output.push('\n');
    output.push_str(field.doc);
    output.push_str("\n\n");
    output.push_str(&format!("**Default value**: `{}`\n", field.default));
    output.push('\n');
    output.push_str(&format!("**Type**: `{}`\n", field.value_type));
    output.push('\n');
    output.push_str(&format!(
        "**Example usage**:\n\n```toml\n[tool.ruff{}]\n{}\n```\n",
        if group_name.is_some() {
            format!(".{}", group_name.unwrap())
        } else {
            String::new()
        },
        field.example
    ));
    output.push('\n');
}

pub fn generate() -> String {
    let mut output: String = "### Top-level\n\n".into();

    let mut sorted_options = Options::get_available_options();
    sorted_options.sort_by_key(|(name, _)| *name);

    // Generate all the top-level fields.
    for (name, entry) in &sorted_options {
        let OptionEntry::Field(field) = entry else { continue; };
        emit_field(&mut output, name, field, None);
        output.push_str("---\n\n");
    }

    // Generate all the sub-groups.
    for (group_name, entry) in &sorted_options {
        let OptionEntry::Group(fields) = entry else { continue; };
        output.push_str(&format!("### `{group_name}`\n"));
        output.push('\n');
        for (name, entry) in fields.iter().sorted_by_key(|(name, _)| name) {
            let OptionEntry::Field(field) = entry else { continue; };
            emit_field(&mut output, name, field, Some(group_name));
            output.push_str("---\n\n");
        }
    }

    output
}
