use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use docx::{
    document::{BodyContent, TableCellContent},
    DocxFile,
};
use std::fs;

pub struct DocxConverter;

impl DocumentConverter for DocxConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".docx" {
                    return None;
                }
            }
        }

        if !fs::metadata(local_path).is_ok() {
            return None;
        }

        let docx_file = DocxFile::from_file(local_path).expect("Failed to read DOCX file");
        let doc = docx_file.parse().unwrap();

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
                        match &cell.content[0] {
                            TableCellContent::Paragraph(paragraph) => {
                                for text in paragraph.iter_text() {
                                    markdown.push_str(&format!(" {} |", text));
                                }
                            }
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
                            for content in cell.content.iter() {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for text in paragraph.iter_text() {
                                            markdown.push_str(&format!(" {} |", text));
                                        }
                                    }
                                }
                            }
                        }
                        markdown.push_str("\n");
                    }
                }
            }
        }

        Some(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
