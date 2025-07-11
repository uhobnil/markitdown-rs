use serde::{Deserialize, Serialize};
use crate::error::MarkitdownError;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentConverterResult {
    pub title: Option<String>,
    pub text_content: String,
}

#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub file_extension: Option<String>,
    pub url: Option<String>,
    pub llm_client: Option<String>,
    pub llm_model: Option<String>,
}

pub trait DocumentConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError>;

    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError>;
}
