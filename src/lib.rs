pub mod docx;
pub mod excel;
pub mod html;
pub mod image;
pub mod llm;
pub mod model;
pub mod pdf;
pub mod pptx;

use docx::DocxConverter;
use excel::ExcelConverter;
use html::HtmlConverter;
use image::ImageConverter;
use infer;
use mime_guess::MimeGuess;
use model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use pdf::PdfConverter;
use pptx::PptxConverter;
use std::{collections::HashMap, path::Path};

pub struct MarkItDown {
    converters: Vec<Box<dyn DocumentConverter>>,
}

impl MarkItDown {
    pub fn new() -> Self {
        let mut md = MarkItDown {
            converters: Vec::new(),
        };

        md.register_converter(Box::new(ExcelConverter));
        md.register_converter(Box::new(HtmlConverter));
        md.register_converter(Box::new(ImageConverter));
        md.register_converter(Box::new(PdfConverter));
        md.register_converter(Box::new(PptxConverter));
        md.register_converter(Box::new(DocxConverter));

        md
    }

    pub fn register_converter(&mut self, converter: Box<dyn DocumentConverter>) {
        self.converters.insert(0, converter);
    }

    fn get_file_type_map() -> HashMap<&'static str, Vec<&'static str>> {
        let mut map = HashMap::new();
        map.insert("application/pdf", vec![".pdf"]);
        map.insert("application/msword", vec![".doc"]);
        map.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            vec![".docx"],
        );
        map.insert("application/vnd.ms-excel", vec![".xls"]);
        map.insert(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            vec![".xlsx"],
        );
        map.insert("text/html", vec![".html", ".htm"]);
        map.insert("image/jpeg", vec![".jpg", ".jpeg"]);
        map.insert("image/png", vec![".png"]);
        map.insert("image/gif", vec![".gif"]);
        map.insert("application/zip", vec![".zip"]);
        map.insert("audio/mpeg", vec![".mp3"]);
        map.insert("audio/wav", vec![".wav"]);
        map.insert("application/xml", vec![".xml", ".rss", ".atom"]);
        map
    }

    pub fn detect_file_type(&self, file_path: &str) -> Option<String> {
        if let Some(kind) = infer::get_from_path(file_path).ok().flatten() {
            return Some(format!(".{}", kind.extension()));
        }

        if let Some(ext) = Path::new(file_path).extension() {
            if let Some(ext_str) = ext.to_str() {
                return Some(format!(".{}", ext_str.to_lowercase()));
            }
        }

        if let Ok(_content) = std::fs::read(file_path) {
            if let Some(mime) = MimeGuess::from_path(file_path).first() {
                let mime_str = mime.to_string();
                if let Some(extensions) = Self::get_file_type_map().get(mime_str.as_str()) {
                    return extensions.first().map(|&ext| ext.to_string());
                }
            }
        }

        None
    }

    pub fn convert(
        &self,
        source: &str,
        mut args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(ref mut options) = args {
            if options.file_extension.is_none() {
                options.file_extension = self.detect_file_type(source);
            }
        } else {
            args = Some(ConversionOptions {
                file_extension: self.detect_file_type(source),
                url: None,
                llm_client: None,
                llm_model: None,
            });
        }

        for converter in &self.converters {
            if let Some(result) = converter.convert(source, args.clone()) {
                return Some(result);
            }
        }
        None
    }
}
