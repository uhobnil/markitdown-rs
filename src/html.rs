use html2md::parse_html;

use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};

pub struct HtmlConverter;

impl DocumentConverter for HtmlConverter {
    fn convert(
        &self,
        local_path: &str,
        kwargs: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &kwargs {
            if let Some(ext) = &opts.file_extension {
                if ext != ".html" && ext != ".htm" {
                    return None;
                }
            }
        }

        match std::fs::read_to_string(local_path) {
            Ok(content) => {
                let markdown = parse_html(&content);
                Some(DocumentConverterResult {
                    title: extract_title(&content),
                    text_content: markdown,
                })
            }
            Err(_) => None,
        }
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        kwargs: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &kwargs {
            if let Some(ext) = &opts.file_extension {
                if ext != ".html" && ext != ".htm" {
                    return None;
                }
            }
        }

        match String::from_utf8(bytes.to_vec()) {
            Ok(content) => {
                let markdown = parse_html(&content);
                Some(DocumentConverterResult {
                    title: extract_title(&content),
                    text_content: markdown,
                })
            }
            Err(_) => None,
        }
    }
}

fn extract_title(html: &str) -> Option<String> {
    use regex::Regex;

    let re = Regex::new(r"(?i)<title(?:\s[^>]*)?>(.*?)</title>").unwrap();
    if let Some(captures) = re.captures(html) {
        return Some(captures[1].trim().to_string());
    }
    None
}
