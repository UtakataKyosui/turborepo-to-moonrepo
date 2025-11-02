// Naming convention utilities

/// Convert PascalCase to snake_case
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_upper = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && !prev_is_upper {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
            prev_is_upper = true;
        } else {
            result.push(ch);
            prev_is_upper = false;
        }
    }

    result
}

/// Convert snake_case to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect()
}

/// Convert snake_case to camelCase
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert model name to table name (pluralize and snake_case)
pub fn to_table_name(model_name: &str) -> String {
    let snake = to_snake_case(model_name);
    pluralize(&snake)
}

/// Simple pluralization (handles common cases)
pub fn pluralize(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    // Handle common irregular plurals
    match word.to_lowercase().as_str() {
        "person" => "people".to_string(),
        "child" => "children".to_string(),
        "man" => "men".to_string(),
        "woman" => "women".to_string(),
        "tooth" => "teeth".to_string(),
        "foot" => "feet".to_string(),
        "mouse" => "mice".to_string(),
        "goose" => "geese".to_string(),
        _ => {
            // Regular pluralization rules
            if word.ends_with("s")
                || word.ends_with("x")
                || word.ends_with("z")
                || word.ends_with("ch")
                || word.ends_with("sh")
            {
                format!("{}es", word)
            } else if word.ends_with("y") {
                // Check if preceded by consonant
                let chars: Vec<char> = word.chars().collect();
                if chars.len() > 1 {
                    let before_y = chars[chars.len() - 2];
                    if !"aeiou".contains(before_y.to_lowercase().next().unwrap()) {
                        format!("{}ies", &word[..word.len() - 1])
                    } else {
                        format!("{}s", word)
                    }
                } else {
                    format!("{}s", word)
                }
            } else if word.ends_with("f") {
                format!("{}ves", &word[..word.len() - 1])
            } else if word.ends_with("fe") {
                format!("{}ves", &word[..word.len() - 2])
            } else {
                format!("{}s", word)
            }
        }
    }
}

/// Simple singularization (reverse of pluralize)
pub fn singularize(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    // Handle common irregular singulars
    match word.to_lowercase().as_str() {
        "people" => "person".to_string(),
        "children" => "child".to_string(),
        "men" => "man".to_string(),
        "women" => "woman".to_string(),
        "teeth" => "tooth".to_string(),
        "feet" => "foot".to_string(),
        "mice" => "mouse".to_string(),
        "geese" => "goose".to_string(),
        _ => {
            // Regular singularization rules
            if word.ends_with("ies") {
                format!("{}y", &word[..word.len() - 3])
            } else if word.ends_with("ves") {
                format!("{}f", &word[..word.len() - 3])
            } else if word.ends_with("ses")
                || word.ends_with("xes")
                || word.ends_with("zes")
                || word.ends_with("ches")
                || word.ends_with("shes")
            {
                word[..word.len() - 2].to_string()
            } else if word.ends_with('s') && word.len() > 1 {
                word[..word.len() - 1].to_string()
            } else {
                word.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("BlogPost"), "blog_post");
        assert_eq!(to_snake_case("HTTPRequest"), "h_t_t_p_request");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("blog_post"), "BlogPost");
        assert_eq!(to_pascal_case("http_request"), "HttpRequest");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("user"), "user");
        assert_eq!(to_camel_case("blog_post"), "blogPost");
        assert_eq!(to_camel_case("http_request"), "httpRequest");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("user"), "users");
        assert_eq!(pluralize("post"), "posts");
        assert_eq!(pluralize("category"), "categories");
        assert_eq!(pluralize("box"), "boxes");
        assert_eq!(pluralize("person"), "people");
    }

    #[test]
    fn test_to_table_name() {
        assert_eq!(to_table_name("User"), "users");
        assert_eq!(to_table_name("BlogPost"), "blog_posts");
        assert_eq!(to_table_name("Category"), "categories");
    }
}
