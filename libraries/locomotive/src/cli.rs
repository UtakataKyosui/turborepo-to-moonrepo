use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lcm")]
#[command(about = "LocoMotive: Generate Loco scaffold commands from OpenAPI specifications with ER diagram support")]
#[command(version = "0.5.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate Loco scaffold commands from OpenAPI file
    Generate {
        /// Path to OpenAPI YAML or JSON file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file for generated commands (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Path to ER diagram file (.md, .mermaid, .mmd)
        #[arg(long)]
        er_diagram: Option<PathBuf>,

        /// Generate API-only scaffolds (adds --api flag)
        #[arg(long, default_value_t = false)]
        api: bool,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Commands)]
        format: OutputFormat,

        /// Generate executable bash script
        #[arg(long, default_value_t = false)]
        script: bool,

        /// Generate markdown report
        #[arg(long, default_value_t = false)]
        report: bool,
    },

    /// Analyze OpenAPI file and show detected resources (dry-run)
    Analyze {
        /// Path to OpenAPI YAML or JSON file
        #[arg(short, long)]
        input: PathBuf,

        /// Path to ER diagram file (.md, .mermaid, .mmd)
        #[arg(long)]
        er_diagram: Option<PathBuf>,

        /// Show detailed field information
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },

    /// Interactive mode - guided generation
    Interactive,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Plain commands (default)
    Commands,
    /// Bash script with error handling
    Script,
    /// Markdown report
    Report,
    /// JSON output for further processing
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Commands => write!(f, "commands"),
            OutputFormat::Script => write!(f, "script"),
            OutputFormat::Report => write!(f, "report"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug)]
pub struct GenerateOptions {
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
    pub er_diagram: Option<PathBuf>,
    pub api_only: bool,
    pub format: OutputFormat,
    pub generate_script: bool,
    pub generate_report: bool,
}

impl GenerateOptions {
    pub fn from_generate_command(cmd: &Commands) -> Option<Self> {
        if let Commands::Generate { 
            input, 
            output, 
            er_diagram,
            api, 
            format, 
            script, 
            report 
        } = cmd {
            Some(GenerateOptions {
                input_file: input.clone(),
                output_file: output.clone(),
                er_diagram: er_diagram.clone(),
                api_only: *api,
                format: format.clone(),
                generate_script: *script,
                generate_report: *report,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct AnalyzeOptions {
    pub input_file: PathBuf,
    pub er_diagram: Option<PathBuf>,
    pub verbose: bool,
}

impl AnalyzeOptions {
    pub fn from_analyze_command(cmd: &Commands) -> Option<Self> {
        if let Commands::Analyze { input, er_diagram, verbose } = cmd {
            Some(AnalyzeOptions {
                input_file: input.clone(),
                er_diagram: er_diagram.clone(),
                verbose: *verbose,
            })
        } else {
            None
        }
    }
}

/// CLI使用例を表示
pub fn print_examples() {
    println!(r#"
LocoMotive Examples:
  # Basic usage - generate commands to stdout
  lcm generate --input api.yaml

  # Generate with ER diagram integration
  lcm generate --input api.yaml --er-diagram schema.md

  # Generate API-only scaffolds with bash script
  lcm generate --input api.yaml --api --script --output setup.sh

  # Generate markdown report with relationships
  lcm generate --input api.yaml --er-diagram schema.md --format report --output report.md

  # Analyze OpenAPI file with ER diagram (dry-run)
  lcm analyze --input api.yaml --er-diagram schema.md --verbose

  # Interactive mode
  lcm interactive

  # Generate JSON output for scripting
  lcm generate --input api.yaml --format json --output data.json
"#);
}

/// バリデーション関数
pub fn validate_input_file(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Input file does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }

    // ファイル拡張子チェック
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if !["yaml", "yml", "json"].contains(&ext.as_str()) {
            return Err(format!("Unsupported file format: {}. Supported formats: yaml, yml, json", ext));
        }
    } else {
        return Err("File must have an extension (.yaml, .yml, or .json)".to_string());
    }

    Ok(())
}

/// 出力ディレクトリの作成
pub fn ensure_output_directory(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_validate_input_file() {
        // 存在しないファイル
        let non_existent = PathBuf::from("non_existent.yaml");
        assert!(validate_input_file(&non_existent).is_err());

        // 正しい拡張子のテストファイル作成
        let temp_dir = tempdir().unwrap();
        let yaml_file = temp_dir.path().join("test.yaml");
        File::create(&yaml_file).unwrap();
        
        assert!(validate_input_file(&yaml_file).is_ok());

        let json_file = temp_dir.path().join("test.json");
        File::create(&json_file).unwrap();
        
        assert!(validate_input_file(&json_file).is_ok());

        // 不正な拡張子
        let txt_file = temp_dir.path().join("test.txt");
        File::create(&txt_file).unwrap();
        
        assert!(validate_input_file(&txt_file).is_err());
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(format!("{}", OutputFormat::Commands), "commands");
        assert_eq!(format!("{}", OutputFormat::Script), "script");
        assert_eq!(format!("{}", OutputFormat::Report), "report");
        assert_eq!(format!("{}", OutputFormat::Json), "json");
    }
}