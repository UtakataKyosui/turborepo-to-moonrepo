mod analyzer;
mod generator;
mod cli;
mod mermaid_parser;

use clap::Parser;
use std::fs;
use std::io::{self, Write};
use serde_json;
use openapiv3::OpenAPI;

use analyzer::OpenAPIAnalyzer;
use generator::LocoScaffoldGenerator;
use cli::{Cli, Commands, GenerateOptions, AnalyzeOptions, OutputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { .. } => {
            let options = GenerateOptions::from_generate_command(&cli.command)
                .ok_or("Failed to parse generate options")?;
            handle_generate(options)?;
        }
        Commands::Analyze { .. } => {
            let options = AnalyzeOptions::from_analyze_command(&cli.command)
                .ok_or("Failed to parse analyze options")?;
            handle_analyze(options)?;
        }
        Commands::Interactive => {
            handle_interactive()?;
        }
    }

    Ok(())
}

fn handle_generate(options: GenerateOptions) -> Result<(), Box<dyn std::error::Error>> {
    // 入力ファイルの検証
    cli::validate_input_file(&options.input_file)?;

    // OpenAPI仕様の読み込み
    let content = fs::read_to_string(&options.input_file)?;
    let openapi: OpenAPI = if options.input_file.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .as_deref() == Some("json") {
        serde_json::from_str(&content)?
    } else {
        serde_yaml::from_str(&content)?
    };

    // ER図がある場合は統合して解析実行
    let analyzer = if let Some(er_path) = &options.er_diagram {
        println!("📋 Loading ER diagram from: {}", er_path.display());
        OpenAPIAnalyzer::with_er_diagram(openapi, er_path.to_str().unwrap())?
    } else {
        OpenAPIAnalyzer::new(openapi)
    };
    
    let resources = analyzer.analyze()?;

    if resources.is_empty() {
        eprintln!("⚠️  No resources detected in the OpenAPI specification");
        return Ok(());
    }

    // 生成実行
    let generator = LocoScaffoldGenerator::new();
    let output = match options.format {
        OutputFormat::Commands => {
            let commands = generator.generate_commands(&resources, options.api_only);
            commands.join("\n")
        }
        OutputFormat::Script => {
            generator.generate_batch_script(&resources, options.api_only)
        }
        OutputFormat::Report => {
            generator.generate_report(&resources)
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(&resources)?
        }
    };

    // 出力
    match options.output_file {
        Some(path) => {
            cli::ensure_output_directory(&path)?;
            fs::write(&path, output)?;
            println!("✅ Output written to: {}", path.display());
        }
        None => {
            println!("{}", output);
        }
    }

    // 統計情報表示
    println!("\n📊 Generation Summary:");
    println!("  - Resources processed: {}", resources.len());
    println!("  - Total fields: {}", resources.iter().map(|r| r.fields.len()).sum::<usize>());
    println!("  - API mode: {}", if options.api_only { "enabled" } else { "disabled" });

    Ok(())
}

fn handle_analyze(options: AnalyzeOptions) -> Result<(), Box<dyn std::error::Error>> {
    // 入力ファイルの検証
    cli::validate_input_file(&options.input_file)?;

    // OpenAPI仕様の読み込み
    let content = fs::read_to_string(&options.input_file)?;
    let openapi: OpenAPI = if options.input_file.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .as_deref() == Some("json") {
        serde_json::from_str(&content)?
    } else {
        serde_yaml::from_str(&content)?
    };

    // ER図がある場合は統合して解析実行
    let analyzer = if let Some(er_path) = &options.er_diagram {
        println!("📋 Loading ER diagram from: {}", er_path.display());
        OpenAPIAnalyzer::with_er_diagram(openapi, er_path.to_str().unwrap())?
    } else {
        OpenAPIAnalyzer::new(openapi)
    };
    
    let resources = analyzer.analyze()?;

    if resources.is_empty() {
        println!("❌ No resources detected in the OpenAPI specification");
        return Ok(());
    }

    // 結果表示
    println!("🔍 OpenAPI Analysis Results\n");
    println!("File: {}", options.input_file.display());
    println!("Resources found: {}\n", resources.len());

    for (i, resource) in resources.iter().enumerate() {
        println!("{}. {}", i + 1, resource.name);
        println!("   Operations: {}", resource.operations.join(", "));
        
        if options.verbose && !resource.fields.is_empty() {
            println!("   Fields:");
            for field in &resource.fields {
                let type_info = match &field.field_type {
                    analyzer::FieldType::References(table) => format!("references:{}", table),
                    _ => format!("{:?}", field.field_type).to_lowercase(),
                };
                let required_mark = if field.required { " (!)" } else { "" };
                println!("     - {}: {}{}", field.name, type_info, required_mark);
            }
        } else if !resource.fields.is_empty() {
            println!("   Fields: {} detected", resource.fields.len());
        }
        println!();
    }

    // 生成予定のコマンド例を表示
    if !resources.is_empty() {
        println!("💡 Example generated command:");
        let generator = LocoScaffoldGenerator::new();
        let example_command = generator.generate_commands(&[resources[0].clone()], true);
        println!("   {}", example_command[0]);
    }

    Ok(())
}

fn handle_interactive() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚂 OpenAPI to Loco Scaffold - Interactive Mode\n");

    // ファイルパス入力
    print!("📁 Enter OpenAPI file path: ");
    io::stdout().flush()?;
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let input_path = std::path::PathBuf::from(input_path.trim());

    // ファイル検証
    if let Err(e) = cli::validate_input_file(&input_path) {
        eprintln!("❌ {}", e);
        return Ok(());
    }

    // API-onlyモード選択
    print!("🔧 Generate API-only scaffolds? [Y/n]: ");
    io::stdout().flush()?;
    let mut api_choice = String::new();
    io::stdin().read_line(&mut api_choice)?;
    let api_only = !api_choice.trim().to_lowercase().starts_with('n');

    // 出力形式選択
    println!("📤 Select output format:");
    println!("  1. Commands only (default)");
    println!("  2. Bash script");
    println!("  3. Markdown report");
    print!("Enter choice [1-3]: ");
    io::stdout().flush()?;
    let mut format_choice = String::new();
    io::stdin().read_line(&mut format_choice)?;
    
    let format = match format_choice.trim() {
        "2" => OutputFormat::Script,
        "3" => OutputFormat::Report,
        _ => OutputFormat::Commands,
    };

    // 処理実行
    let options = GenerateOptions {
        input_file: input_path,
        output_file: None,
        er_diagram: None,
        api_only,
        format,
        generate_script: false,
        generate_report: false,
    };

    println!("\n🔄 Processing...");
    handle_generate(options)?;

    Ok(())
}