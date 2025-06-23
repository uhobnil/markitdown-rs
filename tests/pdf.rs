use markitdown::{
    model::{ConversionOptions, DocumentConverter},
    pdf::PdfConverter,
};

#[test]
fn test_pdf_conversion() {
    let converter = PdfConverter;
    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert("tests/test_files/test.pdf", Some(options));
    assert!(result.is_some());
}

#[test]
fn test_pdf_bytes_conversion() {
    let converter = PdfConverter;
    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert_bytes(include_bytes!("./test_files/test.pdf"), Some(options));
    assert!(result.is_some());
}

