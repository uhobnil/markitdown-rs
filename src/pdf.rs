use pdf_extract;

use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct PdfConverter;

impl DocumentConverter for PdfConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".pdf" {
                    return None;
                }
            }
        }

        let bytes = std::fs::read(local_path).unwrap();
        let text_content = pdf_extract::extract_text_from_mem(&bytes).unwrap();
        Some(DocumentConverterResult {
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
}
