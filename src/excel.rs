use calamine::{open_workbook, Reader, Xlsx};
use std::{io::Cursor, path::Path};

use crate::error::{MarkitdownError, Result};
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct ExcelConverter;

impl DocumentConverter for ExcelConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".xlsx" && ext != ".xls" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .xlsx or .xls file, got {}", ext)
                    ));
                }
            }
        }

        let path = Path::new(local_path);
        println!("Opening file: {:#?}", path);
        let mut workbook: Xlsx<_> = open_workbook(path)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to open Excel file: {}", e)))?;
        let mut markdown = String::new();

        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                .collect();

            if rows.is_empty() {
                return Ok(DocumentConverterResult {
                    title: None,
                    text_content: String::new(),
                });
            }

            markdown.push_str("|");
            for cell in &rows[0] {
                markdown.push_str(&format!(" {} |", cell));
            }
            markdown.push_str("\n|");

            for _ in &rows[0] {
                markdown.push_str(" --- |");
            }
            markdown.push_str("\n");

            for row in rows.iter().skip(1) {
                markdown.push_str("|");
                for cell in row {
                    markdown.push_str(&format!(" {} |", cell));
                }
                markdown.push_str("\n");
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
                if ext != ".xlsx" && ext != ".xls" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .xlsx or .xls file, got {}", ext)
                    ));
                }
            }
        }
        let reader = Cursor::new(bytes);
        let mut workbook: Xlsx<_> = Xlsx::new(reader)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to open Excel file: {}", e)))?;

        let mut markdown = String::new();

        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                .collect();

            if rows.is_empty() {
                return Ok(DocumentConverterResult {
                    title: None,
                    text_content: String::new(),
                });
            }

            markdown.push_str("|");
            for cell in &rows[0] {
                markdown.push_str(&format!(" {} |", cell));
            }
            markdown.push_str("\n|");

            for _ in &rows[0] {
                markdown.push_str(" --- |");
            }
            markdown.push_str("\n");

            for row in rows.iter().skip(1) {
                markdown.push_str("|");
                for cell in row {
                    markdown.push_str(&format!(" {} |", cell));
                }
                markdown.push_str("\n");
            }
        }

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
