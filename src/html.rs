use html2md::parse_html;

use crate::error::MarkitdownError;
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct HtmlConverter;

impl DocumentConverter for HtmlConverter {
    fn convert(
        &self,
        local_path: &str,
        kwargs: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &kwargs {
            if let Some(ext) = &opts.file_extension {
                if ext != ".html" && ext != ".htm" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .html or .htm file, got {}", ext)
                    ));
                }
            }
        }

        let content = std::fs::read_to_string(local_path)?;
        let markdown = parse_html(&content);
        Ok(DocumentConverterResult {
            title: extract_title(&content),
            text_content: markdown,
        })
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        kwargs: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &kwargs {
            if let Some(ext) = &opts.file_extension {
                if ext != ".html" && ext != ".htm" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .html or .htm file, got {}", ext)
                    ));
                }
            }
        }

        let content = String::from_utf8(bytes.to_vec())
            .map_err(|e| MarkitdownError::ParseError(format!("Invalid UTF-8 encoding: {}", e)))?;
        let markdown = parse_html(&content);
        Ok(DocumentConverterResult {
            title: extract_title(&content),
            text_content: markdown,
        })
    }
}

fn extract_title(html: &str) -> Option<String> {
    use regex::Regex;

    let re = Regex::new(r"(?i)<title(?:\s[^>]*)?>(.*?)</title>").ok()?;
    if let Some(captures) = re.captures(html) {
        return Some(captures[1].trim().to_string());
    }
    None
}
