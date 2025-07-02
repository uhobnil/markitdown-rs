use markitdown::{model::ConversionOptions, MarkItDown};

#[test]
fn test_excel_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".xlsx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert("tests/test_files/test.xlsx", Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
    // write_to_file(&result.as_ref().unwrap().text_content);
}

#[test]
fn test_excel_bytes_conversion() {
    let options = ConversionOptions {
        file_extension: Some(".xlsx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let markitdown = MarkItDown::new();

    let result = markitdown.convert_bytes(include_bytes!("./test_files/test.xlsx"), Some(options));
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

fn write_to_file(content: &str) {
    use std::io::Write;
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
