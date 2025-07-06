use crate::error::MarkitdownError;
use crate::llm;
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use exif::Reader;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor};
use tokio;

pub struct ImageConverter;

impl DocumentConverter for ImageConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".jpg" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .jpg file, got {}", ext)
                    ));
                }
            }
        }

        fs::metadata(local_path)?;

        let file = File::open(local_path)?;
        let exif = Reader::new().read_from_container(&mut BufReader::new(&file))
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to read EXIF data: {}", e)))?;

        let mut markdown = String::new();

        for field in exif.fields() {
            markdown.push_str(&format!(
                "{}: {}\n",
                field.tag,
                field.display_value().with_unit(&exif)
            ));
        }

        if let Some(opts) = &args {
            if let Some(llm_client) = &opts.llm_client {
                if let Some(llm_model) = &opts.llm_model {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .map_err(|e| MarkitdownError::Conversion(format!("Failed to create runtime: {}", e)))?;

                    if let Some(llm_description) = rt.block_on(async {
                        llm::get_llm_description(local_path, llm_client, llm_model).await
                    }) {
                        markdown.push_str("\n# Description:\n");
                        markdown.push_str(&llm_description);
                    }
                }
            }
        }

        println!("markdown:{}", markdown);

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".jpg" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .jpg file, got {}", ext)
                    ));
                }
            }
        }

        let exif = Reader::new().read_from_container(&mut Cursor::new(bytes))
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to read EXIF data: {}", e)))?;

        let mut markdown = String::new();

        for field in exif.fields() {
            markdown.push_str(&format!(
                "{}: {}\n",
                field.tag,
                field.display_value().with_unit(&exif)
            ));
        }

        if let Some(opts) = &args {
            if let Some(llm_client) = &opts.llm_client {
                if let Some(llm_model) = &opts.llm_model {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .map_err(|e| MarkitdownError::Conversion(format!("Failed to create runtime: {}", e)))?;

                    if let Some(llm_description) = rt.block_on(async {
                        llm::get_llm_description("", llm_client, llm_model).await
                    }) {
                        markdown.push_str("\n# Description:\n");
                        markdown.push_str(&llm_description);
                    }
                }
            }
        }

        println!("markdown:{}", markdown);

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
