use markitdown::{
    csv::CsvConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_plaintext_conversion() {
    let converter = CsvConverter;
    let options = ConversionOptions {
        file_extension: Some(".csv".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert("tests/test_files/test.csv", Some(options));
    assert!(result.is_some());
}
