use markitdown::{model::ConversionOptions, MarkItDown};

#[test]
fn test_pdf_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test.pdf", Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_pdf_bytes_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert_bytes(include_bytes!("./test_files/test.pdf"), Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
