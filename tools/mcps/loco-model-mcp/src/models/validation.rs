// Validation rule data structures

use serde::{Deserialize, Serialize};

/// Validation rule for a model field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Field name to validate
    pub field: String,

    /// Validation type
    pub rule: ValidationRuleType,
}

/// Types of validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ValidationRuleType {
    /// Email format validation
    Email,

    /// URL format validation
    Url,

    /// Length constraints
    Length {
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<usize>,
    },

    /// Numeric range validation
    Range {
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
    },

    /// Regular expression matching
    Regex { pattern: String },

    /// Custom validation function
    Custom {
        function_name: String,
        message: Option<String>,
    },

    /// Required field (not null)
    Required,

    /// Must be one of specified values
    In { values: Vec<String> },

    /// Must not be one of specified values
    NotIn { values: Vec<String> },
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(field: String, rule: ValidationRuleType) -> Self {
        Self { field, rule }
    }

    /// Create an email validation rule
    pub fn email(field: String) -> Self {
        Self::new(field, ValidationRuleType::Email)
    }

    /// Create a URL validation rule
    pub fn url(field: String) -> Self {
        Self::new(field, ValidationRuleType::Url)
    }

    /// Create a length validation rule
    pub fn length(field: String, min: Option<usize>, max: Option<usize>) -> Self {
        Self::new(field, ValidationRuleType::Length { min, max })
    }

    /// Create a range validation rule
    pub fn range(field: String, min: Option<f64>, max: Option<f64>) -> Self {
        Self::new(field, ValidationRuleType::Range { min, max })
    }

    /// Create a regex validation rule
    pub fn regex(field: String, pattern: String) -> Self {
        Self::new(field, ValidationRuleType::Regex { pattern })
    }

    /// Create a required field validation rule
    pub fn required(field: String) -> Self {
        Self::new(field, ValidationRuleType::Required)
    }

    /// Generate validator crate attribute code
    pub fn to_validator_attribute(&self) -> String {
        match &self.rule {
            ValidationRuleType::Email => "#[validate(email)]".to_string(),
            ValidationRuleType::Url => "#[validate(url)]".to_string(),
            ValidationRuleType::Length { min, max } => {
                let mut parts = Vec::new();
                if let Some(min_val) = min {
                    parts.push(format!("min = {}", min_val));
                }
                if let Some(max_val) = max {
                    parts.push(format!("max = {}", max_val));
                }
                format!("#[validate(length({}))]", parts.join(", "))
            }
            ValidationRuleType::Range { min, max } => {
                let mut parts = Vec::new();
                if let Some(min_val) = min {
                    parts.push(format!("min = {}", min_val));
                }
                if let Some(max_val) = max {
                    parts.push(format!("max = {}", max_val));
                }
                format!("#[validate(range({}))]", parts.join(", "))
            }
            ValidationRuleType::Regex { pattern } => {
                format!("#[validate(regex = \"{}\")]", pattern)
            }
            ValidationRuleType::Custom {
                function_name,
                message,
            } => {
                let msg = message
                    .as_ref()
                    .map(|m| format!(", message = \"{}\"", m))
                    .unwrap_or_default();
                format!("#[validate(custom = \"{}\"{}", function_name, msg)
            }
            ValidationRuleType::Required => "".to_string(), // Handled by nullable field property
            ValidationRuleType::In { values } => {
                format!("#[validate(contains = {:?})]", values)
            }
            ValidationRuleType::NotIn { values } => {
                format!("#[validate(does_not_contain = {:?})]", values)
            }
        }
    }
}
