use pdf_extract;

use crate::error::{MarkitdownError, Result};
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct PdfConverter;

impl DocumentConverter for PdfConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".pdf" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .pdf file, got {}", ext)
                    ));
                }
            }
        }

        let bytes = std::fs::read(local_path)?;
        let text_content = pdf_extract::extract_text_from_mem(&bytes)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to extract text from PDF: {}", e)))?;
        Ok(DocumentConverterResult {
            title: None,
            text_content,
        })

        // match Document::load(local_path) {
        //     Ok(doc) => {
        //         let mut text_content = String::new();

        //         for page_num in 1..=doc.get_pages().len() {
        //             if let Ok(text) = doc.extract_text(&[page_num.try_into().unwrap()]) {
        //                 text_content.push_str(&text);
        //                 text_content.push_str("\n\n");
        //             }
        //         }

        //         Some(DocumentConverterResult {
        //             title: None,
        //             text_content,
        //         })
        //     }
        //     Err(_) => None,
        // }
    }
    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".pdf" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .pdf file, got {}", ext)
                    ));
                }
            }
        }

        let text_content = pdf_extract::extract_text_from_mem(bytes)
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to extract text from PDF: {}", e)))?;
        Ok(DocumentConverterResult {
            title: None,
            text_content,
        })
    }
}
