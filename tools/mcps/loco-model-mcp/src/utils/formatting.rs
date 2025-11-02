// Code formatting utilities

/// Add proper indentation to code
pub fn indent(code: &str, level: usize) -> String {
    let indent_str = "    ".repeat(level);
    code.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format a Rust type with Option wrapper if nullable
pub fn format_rust_type(base_type: &str, nullable: bool) -> String {
    if nullable {
        format!("Option<{}>", base_type)
    } else {
        base_type.to_string()
    }
}

/// Format a list of items as a Rust array or vec
pub fn format_rust_array(items: &[String]) -> String {
    if items.is_empty() {
        "vec![]".to_string()
    } else {
        format!("vec![{}]", items.join(", "))
    }
}

/// Format a doc comment for Rust code
pub fn format_doc_comment(text: &str) -> String {
    text.lines()
        .map(|line| format!("/// {}", line.trim()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Wrap text in quotes (for string literals)
pub fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\\\""))
}

/// Format field attributes for SeaORM
pub fn format_field_attributes(
    nullable: bool,
    unique: bool,
    primary_key: bool,
    indexed: bool,
) -> Vec<String> {
    let mut attrs = Vec::new();

    if primary_key {
        attrs.push("#[sea_orm(primary_key)]".to_string());
    }

    if unique {
        attrs.push("#[sea_orm(unique)]".to_string());
    }

    if indexed && !unique && !primary_key {
        attrs.push("#[sea_orm(indexed)]".to_string());
    }

    if nullable {
        attrs.push("#[sea_orm(nullable)]".to_string());
    }

    attrs
}

/// Join lines with proper newline handling
pub fn join_lines(lines: &[String]) -> String {
    lines.join("\n")
}

/// Remove empty lines from code
pub fn remove_empty_lines(code: &str) -> String {
    code.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Add blank line between sections
pub fn add_section_spacing(sections: &[String]) -> String {
    sections.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        let code = "fn test() {\n    println!(\"hello\");\n}";
        let indented = indent(code, 1);
        assert!(indented.starts_with("    fn test()"));
    }

    #[test]
    fn test_format_rust_type() {
        assert_eq!(format_rust_type("String", false), "String");
        assert_eq!(format_rust_type("String", true), "Option<String>");
    }

    #[test]
    fn test_quote() {
        assert_eq!(quote("hello"), "\"hello\"");
        assert_eq!(quote("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_format_field_attributes() {
        let attrs = format_field_attributes(false, false, true, false);
        assert_eq!(attrs, vec!["#[sea_orm(primary_key)]"]);

        let attrs = format_field_attributes(true, true, false, false);
        assert_eq!(
            attrs,
            vec!["#[sea_orm(unique)]", "#[sea_orm(nullable)]"]
        );
    }
}
