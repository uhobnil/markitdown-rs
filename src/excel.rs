use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct ExcelConverter;

impl DocumentConverter for ExcelConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".xlsx" && ext != ".xls" {
                    return None;
                }
            }
        }

        let path = Path::new(local_path);
        println!("Opening file: {:#?}", path);
        let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
        let mut markdown = String::new();

        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                .collect();

            if rows.is_empty() {
                return Some(DocumentConverterResult {
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

        Some(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
