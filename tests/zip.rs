use markitdown::{
    model::{ConversionOptions, DocumentConverter},
    MarkItDown,
};

#[test]
fn test_excel_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".zip".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test.zip", Some(options));
    assert!(result.is_some());
}
