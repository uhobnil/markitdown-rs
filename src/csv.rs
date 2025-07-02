use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use crate::error::{MarkitdownError, Result};
use csv::ReaderBuilder;
use std::fs::File;

pub struct CsvConverter;

impl DocumentConverter for CsvConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".csv" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .csv file, got {}", ext)
                    ));
                }
            }
        }

        let mut markdown = String::new();
        let file = File::open(local_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let rc: String = record
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&str>>()
                        .join(",");
                    markdown.push_str(&rc);
                    markdown.push_str("\n");
                }
                Err(err) => {
                    markdown.push_str(&format!("{:?}\n", err));
                }
            }
        }
        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".csv" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .csv file, got {}", ext)
                    ));
                }
            }
        }
        let mut markdown = String::new();
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(bytes);
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let rc: String = record
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&str>>()
                        .join(",");
                    markdown.push_str(&rc);
                    markdown.push_str("\n");
                }
                Err(err) => {
                    markdown.push_str(&format!("{:?}\n", err));
                }
            }
        }
        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
