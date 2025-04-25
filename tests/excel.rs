use markitdown::{
    excel::ExcelConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_excel_conversion() {
    let converter = ExcelConverter;
    let options = ConversionOptions {
        file_extension: Some(".xlsx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert("tests/test_files/test.xlsx", Some(options));
    // write_to_file(&result.as_ref().unwrap().text_content);
    assert!(result.is_some());
}

#[test]
fn test_excel_bytes_conversion() {
    let converter = ExcelConverter;
    let options = ConversionOptions {
        file_extension: Some(".xlsx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert_bytes(include_bytes!("./test_files/test.xlsx"), Some(options));
    assert!(result.is_some());
}

fn write_to_file(content: &str) {
    use std::io::Write;
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
