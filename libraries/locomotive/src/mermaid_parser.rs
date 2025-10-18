use crate::analyzer::{RelationshipInfo, RelationshipType};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MermaidParseError {
    pub message: String,
}

impl fmt::Display for MermaidParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mermaid parse error: {}", self.message)
    }
}

impl Error for MermaidParseError {}

pub struct MermaidErParser;

impl MermaidErParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse Mermaid ER diagram from text content
    pub fn parse_er_diagram(&self, content: &str) -> Result<Vec<RelationshipInfo>, Box<dyn Error>> {
        let mut relationships = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_mermaid_block = false;
        let mut mermaid_content = String::new();

        // Extract Mermaid content from markdown code blocks
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("```mermaid") || trimmed == "```mermaid" {
                in_mermaid_block = true;
                continue;
            } else if trimmed == "```" && in_mermaid_block {
                in_mermaid_block = false;
                // Process the collected mermaid content
                let parsed_rels = self.parse_mermaid_content(&mermaid_content)?;
                relationships.extend(parsed_rels);
                mermaid_content.clear();
            } else if in_mermaid_block {
                mermaid_content.push_str(line);
                mermaid_content.push('\n');
            }
        }

        // If no code block markers found, treat entire content as Mermaid
        if relationships.is_empty() && !content.trim().is_empty() {
            relationships = self.parse_mermaid_content(content)?;
        }

        Ok(relationships)
    }

    /// Parse pure Mermaid ER diagram content
    fn parse_mermaid_content(&self, content: &str) -> Result<Vec<RelationshipInfo>, Box<dyn Error>> {
        let mut relationships = Vec::new();
        
        // ER diagram relationship patterns
        let relationship_patterns = vec![
            // One-to-One: ||--|| or }|--||
            (Regex::new(r"(\w+)\s*\|\|--\|\|\s*(\w+)(?:\s*:\s*(.+))?")?, RelationshipType::OneToOne),
            (Regex::new(r"(\w+)\s*\}\|--\|\|\s*(\w+)(?:\s*:\s*(.+))?")?, RelationshipType::OneToOne),
            // One-to-Many: ||--o{
            (Regex::new(r"(\w+)\s*\|\|--o\{\s*(\w+)(?:\s*:\s*(.+))?")?, RelationshipType::OneToMany),
            // Many-to-One: }o--||
            (Regex::new(r"(\w+)\s*\}o--\|\|\s*(\w+)(?:\s*:\s*(.+))?")?, RelationshipType::ManyToOne),
            // Many-to-Many: }o--o{
            (Regex::new(r"(\w+)\s*\}o--o\{\s*(\w+)(?:\s*:\s*(.+))?")?, RelationshipType::ManyToMany),
        ];

        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("%%") || line.starts_with("erDiagram") {
                continue;
            }

            // Try to match relationship patterns
            for (pattern, rel_type) in &relationship_patterns {
                if let Some(captures) = pattern.captures(line) {
                    let from_table = captures.get(1).unwrap().as_str().to_string();
                    let to_table = captures.get(2).unwrap().as_str().to_string();
                    let description = captures.get(3).map(|m| m.as_str().to_string());

                    relationships.push(RelationshipInfo {
                        from_table,
                        to_table,
                        relationship_type: rel_type.clone(),
                        from_field: None, // Will be inferred later
                        to_field: None,   // Will be inferred later
                        constraint: description,
                    });
                    break;
                }
            }

            // Parse entity definitions with attributes
            if let Some(caps) = Regex::new(r"(\w+)\s*\{")?.captures(line) {
                // Entity definition start - could extract field information here
                continue;
            }
        }

        Ok(relationships)
    }

    /// Parse relationship information from markdown files
    pub fn parse_markdown_file(&self, file_path: &str) -> Result<Vec<RelationshipInfo>, Box<dyn Error>> {
        let content = std::fs::read_to_string(file_path)?;
        self.parse_er_diagram(&content)
    }

    /// Parse relationship information from .mermaid/.mmd files
    pub fn parse_mermaid_file(&self, file_path: &str) -> Result<Vec<RelationshipInfo>, Box<dyn Error>> {
        let content = std::fs::read_to_string(file_path)?;
        self.parse_mermaid_content(&content)
    }

    /// Infer field relationships based on common naming conventions
    pub fn infer_relationship_fields(&self, relationships: &mut Vec<RelationshipInfo>) {
        for relationship in relationships.iter_mut() {
            // Infer foreign key field names based on table names
            if relationship.from_field.is_none() && relationship.to_field.is_none() {
                match relationship.relationship_type {
                    RelationshipType::OneToMany | RelationshipType::ManyToOne => {
                        // Assume foreign key in "many" side table
                        let (fk_table, pk_table) = match relationship.relationship_type {
                            RelationshipType::OneToMany => (&relationship.to_table, &relationship.from_table),
                            RelationshipType::ManyToOne => (&relationship.from_table, &relationship.to_table),
                            _ => continue,
                        };
                        
                        relationship.from_field = Some(format!("{}_id", pk_table.to_lowercase()));
                        relationship.to_field = Some("id".to_string());
                    }
                    RelationshipType::ManyToMany => {
                        // Many-to-many typically uses junction table
                        relationship.constraint = Some(format!(
                            "Junction table: {}_{}", 
                            relationship.from_table.to_lowercase(),
                            relationship.to_table.to_lowercase()
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_er_relationship() {
        let parser = MermaidErParser::new();
        let content = r#"
erDiagram
    USER ||--o{ POST : creates
    POST }o--|| CATEGORY : belongs_to
"#;
        
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 2);
        
        assert_eq!(relationships[0].from_table, "USER");
        assert_eq!(relationships[0].to_table, "POST");
        assert!(matches!(relationships[0].relationship_type, RelationshipType::OneToMany));
    }

    #[test]
    fn test_parse_markdown_with_mermaid() {
        let parser = MermaidErParser::new();
        let content = r#"
# Database Design

```mermaid
erDiagram
    USER ||--o{ POST : creates
```

Some other content.
"#;
        
        let relationships = parser.parse_er_diagram(content).unwrap();
        assert_eq!(relationships.len(), 1);
        assert_eq!(relationships[0].from_table, "USER");
        assert_eq!(relationships[0].to_table, "POST");
    }

    #[test]
    fn test_infer_relationship_fields() {
        let parser = MermaidErParser::new();
        let mut relationships = vec![
            RelationshipInfo {
                from_table: "User".to_string(),
                to_table: "Post".to_string(),
                relationship_type: RelationshipType::OneToMany,
                from_field: None,
                to_field: None,
                constraint: None,
            }
        ];

        parser.infer_relationship_fields(&mut relationships);
        
        assert_eq!(relationships[0].from_field, Some("user_id".to_string()));
        assert_eq!(relationships[0].to_field, Some("id".to_string()));
    }

    #[test]
    fn test_complex_er_relationships() {
        let parser = MermaidErParser::new();
        let content = r#"
erDiagram
    USER ||--o{ POST : creates
    USER ||--o{ COMMENT : writes  
    POST ||--o{ COMMENT : has
    POST }o--|| CATEGORY : belongs_to
    USER }|--|| PROFILE : has
    USER }o--o{ TAG : follows
"#;
        
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 6);
        
        // Check specific relationship types
        assert!(matches!(relationships[0].relationship_type, RelationshipType::OneToMany));
        assert!(matches!(relationships[4].relationship_type, RelationshipType::OneToOne));
        assert!(matches!(relationships[5].relationship_type, RelationshipType::ManyToMany));
    }

    #[test]
    fn test_many_to_many_field_inference() {
        let parser = MermaidErParser::new();
        let mut relationships = vec![
            RelationshipInfo {
                from_table: "User".to_string(),
                to_table: "Tag".to_string(),
                relationship_type: RelationshipType::ManyToMany,
                from_field: None,
                to_field: None,
                constraint: None,
            }
        ];

        parser.infer_relationship_fields(&mut relationships);
        
        assert!(relationships[0].constraint.is_some());
        assert!(relationships[0].constraint.as_ref().unwrap().contains("user_tag"));
    }

    #[test]
    fn test_parse_empty_content() {
        let parser = MermaidErParser::new();
        let relationships = parser.parse_mermaid_content("").unwrap();
        assert_eq!(relationships.len(), 0);
    }

    #[test]
    fn test_parse_invalid_mermaid() {
        let parser = MermaidErParser::new();
        let content = "This is not a valid mermaid diagram";
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 0);
    }

    #[test]
    fn test_relationship_type_parsing() {
        let parser = MermaidErParser::new();
        
        // Test One-to-One
        let content = "erDiagram\n    USER ||--|| PROFILE";
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 1);
        assert!(matches!(relationships[0].relationship_type, RelationshipType::OneToOne));
        
        // Test One-to-Many
        let content = "erDiagram\n    USER ||--o{ POST";
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 1);
        assert!(matches!(relationships[0].relationship_type, RelationshipType::OneToMany));
        
        // Test Many-to-One
        let content = "erDiagram\n    POST }o--|| USER";
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 1);
        assert!(matches!(relationships[0].relationship_type, RelationshipType::ManyToOne));
        
        // Test Many-to-Many
        let content = "erDiagram\n    USER }o--o{ TAG";
        let relationships = parser.parse_mermaid_content(content).unwrap();
        assert_eq!(relationships.len(), 1);
        assert!(matches!(relationships[0].relationship_type, RelationshipType::ManyToMany));
    }
}