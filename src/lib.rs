pub mod csv;
pub mod docx;
pub mod error;
pub mod excel;
pub mod html;
pub mod image;
pub mod llm;
pub mod model;
pub mod pdf;
pub mod pptx;
pub mod rss;

use csv::CsvConverter;
use docx::DocxConverter;
use error::{MarkitdownError, Result};
use excel::ExcelConverter;
use html::HtmlConverter;
use image::ImageConverter;
use infer;
use mime_guess::MimeGuess;
use model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use pdf::PdfConverter;
use pptx::PptxConverter;
use rss::RssConverter;
use std::io::Cursor;
use std::io::Read;
use std::{collections::HashMap, path::Path};
use std::{fs, io};
use tempfile::tempdir;
use zip::ZipArchive;

pub struct MarkItDown {
    converters: Vec<Box<dyn DocumentConverter>>,
}

impl MarkItDown {
    pub fn new() -> Self {
        let mut md = MarkItDown {
            converters: Vec::new(),
        };

        md.register_converter(Box::new(CsvConverter));
        md.register_converter(Box::new(ExcelConverter));
        md.register_converter(Box::new(HtmlConverter));
        md.register_converter(Box::new(ImageConverter));
        md.register_converter(Box::new(RssConverter));
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

    pub fn detect_bytes(&self, bytes: &[u8]) -> Option<String> {
        if let Some(kind) = infer::get(bytes) {
            return Some(format!(".{}", kind.extension()));
        }

        None
    }

    pub fn convert(
        &self,
        source: &str,
        mut args: Option<ConversionOptions>,
    ) -> Result<Option<DocumentConverterResult>> {
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

        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext == ".zip" {
                    let data = fs::read(source)?;
                    let cursor = Cursor::new(data);
                    let mut archive = ZipArchive::new(cursor)?;

                    let zip_name = Path::new(source)
                        .file_name()
                        .ok_or_else(|| {
                            MarkitdownError::InvalidFile("No filename found".to_string())
                        })?
                        .to_str()
                        .ok_or_else(|| {
                            MarkitdownError::InvalidFile("Invalid filename encoding".to_string())
                        })?;
                    let mut markdown =
                        String::from(format!("Content from the zip file {}\n", zip_name).as_str());

                    for i in 0..archive.len() {
                        let mut file = archive.by_index(i)?;
                        let file_name = file.name().to_string();
                        let dir = tempdir()?;
                        let file_path = dir.path().join(&file_name);
                        let mut temp_file = fs::File::create(&file_path)?;
                        io::copy(&mut file, &mut temp_file)?;
                        for converter in &self.converters {
                            let file_args = Some(ConversionOptions {
                                file_extension: self.detect_file_type(
                                    file_path.to_str().ok_or_else(|| {
                                        MarkitdownError::InvalidFile(
                                            "Invalid path encoding".to_string(),
                                        )
                                    })?,
                                ),
                                url: None,
                                llm_client: None,
                                llm_model: None,
                            });
                            match converter.convert(
                                file_path.to_str().ok_or_else(|| {
                                    MarkitdownError::InvalidFile(
                                        "Invalid path encoding".to_string(),
                                    )
                                })?,
                                file_args.clone(),
                            ) {
                                Ok(result) => {
                                    markdown
                                        .push_str(format!("\n## File: {}\n\n", &file_name).as_str());
                                    markdown.push_str(format!("{}\n", result.text_content).as_str());
                                }
                                Err(_) => {} // Skip if converter can't handle this file
                            }
                        }

                        std::fs::remove_file(&file_path)?;
                    }
                    return Ok(Some(DocumentConverterResult {
                        title: None,
                        text_content: markdown,
                    }));
                }
            }
        }

        for converter in &self.converters {
            match converter.convert(source, args.clone()) {
                Ok(result) => return Ok(Some(result)),
                Err(_) => continue, // Try next converter
            }
        }
        Ok(None)
    }

    pub fn convert_bytes(
        &self,
        bytes: &[u8],
        mut args: Option<ConversionOptions>,
    ) -> Result<Option<DocumentConverterResult>> {
        if let Some(ref mut options) = args {
            if options.file_extension.is_none() {
                options.file_extension = self.detect_bytes(bytes);
            }
        } else {
            args = Some(ConversionOptions {
                file_extension: self.detect_bytes(bytes),
                url: None,
                llm_client: None,
                llm_model: None,
            });
        }

        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext == ".zip" {
                    let cursor = Cursor::new(bytes);
                    let mut archive = ZipArchive::new(cursor)?;

                    let mut markdown = String::from("");

                    for i in 0..archive.len() {
                        let mut file = archive.by_index(i)?;
                        let file_name = file.name().to_string();

                        // Read file contents into memory
                        let mut file_contents = Vec::new();
                        file.read_to_end(&mut file_contents)?;

                        for converter in &self.converters {
                            let file_args = Some(ConversionOptions {
                                file_extension: self.detect_file_type(&file_name),
                                url: None,
                                llm_client: None,
                                llm_model: None,
                            });
                            match converter.convert_bytes(&file_contents, file_args.clone()) {
                                Ok(result) => {
                                    markdown
                                        .push_str(format!("\n## File: {}\n\n", &file_name).as_str());
                                    markdown.push_str(format!("{}\n", result.text_content).as_str());
                                }
                                Err(_) => {} // Skip if converter can't handle this file
                            }
                        }
                    }
                    return Ok(Some(DocumentConverterResult {
                        title: None,
                        text_content: markdown,
                    }));
                }
            }
        }

        for converter in &self.converters {
            match converter.convert_bytes(bytes, args.clone()) {
                Ok(result) => return Ok(Some(result)),
                Err(_) => continue, // Try next converter
            }
        }
        Ok(None)
    }
}
