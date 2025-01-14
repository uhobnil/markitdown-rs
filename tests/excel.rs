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
    };

    let result = converter.convert("tests/test_files/test.xlsx", Some(options));
    assert!(result.is_some());
}
