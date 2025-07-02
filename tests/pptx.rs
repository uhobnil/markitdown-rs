use markitdown::{model::ConversionOptions, MarkItDown};

#[test]
fn test_pptx_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".pptx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test.pptx", Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_pptx_bytes_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".pptx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert_bytes(include_bytes!("./test_files/test.pptx"), Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
