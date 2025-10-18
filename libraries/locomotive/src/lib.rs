pub mod analyzer;
pub mod generator;
pub mod cli;
pub mod mermaid_parser;

// Re-export main types for easier access
pub use analyzer::{OpenAPIAnalyzer, ResourceInfo, FieldInfo, FieldType, RelationshipInfo, RelationshipType};
pub use generator::LocoScaffoldGenerator;
pub use cli::{Cli, Commands, OutputFormat, GenerateOptions, AnalyzeOptions};
pub use mermaid_parser::MermaidErParser;