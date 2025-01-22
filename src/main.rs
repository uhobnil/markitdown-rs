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

fn main() {
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
                "".to_string()
            }
        }
        None => "".to_string(),
    };

    let input_file = cli.input.trim().to_string();

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
        }),
    );

    if output == "console" {
        println!("{}", &result.as_ref().unwrap().text_content);
    } else {
        fs::write(output, &result.as_ref().unwrap().text_content).expect("Could not write output");
    }
}
