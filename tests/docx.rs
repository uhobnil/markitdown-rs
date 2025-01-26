use markitdown::{
    docx::DocxConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_docx_conversion() {
    let converter = DocxConverter;
    let options = ConversionOptions {
        file_extension: Some(".docx".to_string()),
        url: None,
    };

    let result = converter.convert("tests/test_files/test.docx", Some(options));
    assert!(result.is_some());
}
