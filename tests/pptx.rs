use markitdown::{
    model::{ConversionOptions, DocumentConverter},
    pptx::PptxConverter,
};

#[test]
fn test_pptx_conversion() {
    let converter = PptxConverter;
    let options = ConversionOptions {
        file_extension: Some(".pptx".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert("tests/test_files/test.pptx", Some(options));
    assert!(result.is_some());
}
