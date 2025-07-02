use clap::{command, Parser};
use std::fs;

use markitdown::{model::ConversionOptions, MarkItDown};

#[derive(Parser, Debug)]
#[command(name = "markitdown")]
struct Cli {
    #[arg(value_name = "FILE", index = 1)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    format: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let output = match cli.output {
        Some(file) => file,
        None => "console".to_string(),
    };

    let format = match cli.format {
        Some(format) => {
            if format == "html" || format == "xlsx" || format == "pdf" {
                format
            } else {
                eprintln!(
                    "Warning: Unsupported format '{}'. Using auto-detection.",
                    format
                );
                "".to_string()
            }
        }
        None => "".to_string(),
    };

    let input_file = cli.input.trim().to_string();

    if !std::path::Path::new(&input_file).exists() {
        return Err(format!("Error: File '{}' not found", input_file).into());
    }

    let markitdown = MarkItDown::new();

    let result = markitdown.convert(
        &input_file,
        Some(ConversionOptions {
            file_extension: if format.is_empty() {
                None
            } else {
                Some(format!(".{}", format))
            },
            url: None,
            llm_client: None,
            llm_model: None,
        }),
    )?;

    if let Some(doc_result) = result {
        if output == "console" {
            println!("{}", &doc_result.text_content);
        } else {
            fs::write(&output, &doc_result.text_content)
                .map_err(|e| format!("Failed to write to '{}': {}", output, e))?;
            eprintln!("Successfully converted to: {}", output);
        }
    } else {
        eprintln!(
            "Error: Unable to convert file '{}'. The file format may not be supported.",
            input_file
        );
        std::process::exit(1);
    }
    Ok(())
}
