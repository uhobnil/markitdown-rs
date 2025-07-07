use markitdown::{model::ConversionOptions, MarkItDown};

#[test]
fn test_html_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".html".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test_blog.html", Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_html_bytes_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".html".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result =
        markitdown.convert_bytes(include_bytes!("./test_files/test_blog.html"), Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
