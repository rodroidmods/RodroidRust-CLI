use crate::error::{CliError, Result};

pub fn validate_package_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(CliError::InvalidPackageName {
            name: name.to_string(),
            reason: "package name cannot be empty".to_string(),
        });
    }

    let parts: Vec<&str> = name.split('.').collect();

    if parts.len() < 2 {
        return Err(CliError::InvalidPackageName {
            name: name.to_string(),
            reason: "package name must have at least two segments".to_string(),
        });
    }

    for part in parts {
        if part.is_empty() {
            return Err(CliError::InvalidPackageName {
                name: name.to_string(),
                reason: "package name contains empty segment".to_string(),
            });
        }
        if !is_valid_identifier(part) {
            return Err(CliError::InvalidPackageName {
                name: name.to_string(),
                reason: format!("'{}' is not a valid Java identifier", part),
            });
        }
        if is_java_keyword(part) {
            return Err(CliError::InvalidPackageName {
                name: name.to_string(),
                reason: format!("'{}' is a reserved Java keyword", part),
            });
        }
    }

    Ok(())
}

fn is_valid_identifier(part: &str) -> bool {
    let mut chars = part.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_java_keyword(word: &str) -> bool {
    matches!(
        word,
        "abstract"
            | "assert"
            | "boolean"
            | "break"
            | "byte"
            | "case"
            | "catch"
            | "char"
            | "class"
            | "const"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extends"
            | "final"
            | "finally"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "implements"
            | "import"
            | "instanceof"
            | "int"
            | "interface"
            | "long"
            | "native"
            | "new"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "return"
            | "short"
            | "static"
            | "strictfp"
            | "super"
            | "switch"
            | "synchronized"
            | "this"
            | "throw"
            | "throws"
            | "transient"
            | "try"
            | "void"
            | "volatile"
            | "while"
            | "true"
            | "false"
            | "null"
    )
}
