use markitdown::{
    model::{ConversionOptions, DocumentConverter},
    rss::RssConverter,
};

#[test]
fn test_rss_conversion() {
    let converter = RssConverter;
    let options = ConversionOptions {
        file_extension: Some(".xml".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    let result = converter.convert("tests/test_files/test.xml", Some(options));
    assert!(result.is_some());
}
