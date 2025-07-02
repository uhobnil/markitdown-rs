use crate::error::{MarkitdownError, Result};
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use docx_rust::{
    document::{BodyContent, TableCellContent, TableRowContent},
    DocxFile,
};
use std::fs;
use std::io::Cursor;

pub struct DocxConverter;

impl DocumentConverter for DocxConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".docx" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .docx file, got {}", ext)
                    ));
                }
            }
        }

        fs::metadata(local_path)?;

        let docx_file = DocxFile::from_file(local_path)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to read DOCX file: {}", e)))?;
        let doc = docx_file.parse()
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to parse DOCX file: {}", e)))?;

        let mut markdown = String::new();

        for content in doc.document.body.content {
            match content {
                BodyContent::Paragraph(paragraph) => {
                    for text in paragraph.iter_text() {
                        markdown.push_str(&text.to_string());
                        markdown.push_str("\n");
                    }
                }
                BodyContent::Table(table) => {
                    markdown.push_str("|");
                    for cell in table.rows[0].cells.iter() {
                        match &cell {
                            TableRowContent::TableCell(tc) => {
                                tc.content.iter().for_each(|content| match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for text in paragraph.iter_text() {
                                            markdown.push_str(&format!(" {} |", text));
                                        }
                                    }
                                })
                            }
                            _ => {}
                        }
                    }
                    markdown.push_str("\n|");

                    for _ in table.rows[0].cells.iter() {
                        markdown.push_str(" --- |");
                    }
                    markdown.push_str("\n");

                    for row in table.rows.iter().skip(1) {
                        markdown.push_str("|");
                        for cell in row.cells.iter() {
                            match &cell {
                                TableRowContent::TableCell(tc) => {
                                    tc.content.iter().for_each(|content| match content {
                                        TableCellContent::Paragraph(paragraph) => {
                                            for text in paragraph.iter_text() {
                                                markdown.push_str(&format!(" {} |", text));
                                            }
                                        }
                                    })
                                }
                                _ => {}
                            }
                        }
                        markdown.push_str("\n");
                    }
                }
                _ => {}
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
                if ext != ".docx" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .docx file, got {}", ext)
                    ));
                }
            }
        }

        let reader = Cursor::new(bytes);

        let docx_file = DocxFile::from_reader(reader)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to read DOCX file: {}", e)))?;
        let doc = docx_file.parse()
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to parse DOCX file: {}", e)))?;

        let mut markdown = String::new();

        for content in doc.document.body.content {
            match content {
                BodyContent::Paragraph(paragraph) => {
                    for text in paragraph.iter_text() {
                        markdown.push_str(&text.to_string());
                        markdown.push_str("\n");
                    }
                }
                BodyContent::Table(table) => {
                    markdown.push_str("|");
                    for cell in table.rows[0].cells.iter() {
                        match &cell {
                            TableRowContent::TableCell(tc) => {
                                tc.content.iter().for_each(|content| match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for text in paragraph.iter_text() {
                                            markdown.push_str(&format!(" {} |", text));
                                        }
                                    }
                                })
                            }
                            _ => {}
                        }
                    }
                    markdown.push_str("\n|");

                    for _ in table.rows[0].cells.iter() {
                        markdown.push_str(" --- |");
                    }
                    markdown.push_str("\n");

                    for row in table.rows.iter().skip(1) {
                        markdown.push_str("|");
                        for cell in row.cells.iter() {
                            match &cell {
                                TableRowContent::TableCell(tc) => {
                                    tc.content.iter().for_each(|content| match content {
                                        TableCellContent::Paragraph(paragraph) => {
                                            for text in paragraph.iter_text() {
                                                markdown.push_str(&format!(" {} |", text));
                                            }
                                        }
                                    })
                                }
                                _ => {}
                            }
                        }
                        markdown.push_str("\n");
                    }
                }
                _ => {}
            }
        }

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
