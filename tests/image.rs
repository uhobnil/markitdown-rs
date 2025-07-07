use markitdown::{model::ConversionOptions, MarkItDown};

#[test]
fn test_image_conversion() {
    let mut options = ConversionOptions {
        file_extension: Some(".jpg".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    if let Ok(_) = std::env::var("GEMINI_API_KEY") {
        options.llm_client = Some("gemini".to_string());
        options.llm_model = Some("gemini-2.0-flash".to_string());
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test.jpg", Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_image_bytes_conversion() {
    let mut options = ConversionOptions {
        file_extension: Some(".jpg".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    if std::env::var("GEMINI_API_KEY").is_ok() {
        options.llm_client = Some("gemini".to_string());
        options.llm_model = Some("gemini-2.0-flash".to_string());
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert_bytes(include_bytes!("./test_files/test.jpg"), Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
