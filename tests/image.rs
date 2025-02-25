use markitdown::{
    image::ImageConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_image_conversion() {
    let converter = ImageConverter;
    let mut options = ConversionOptions {
        file_extension: Some(".jpg".to_string()),
        url: None,
        llm_client: None,
        llm_model: None,
    };

    if let Ok(_) =  std::env::var("GEMINI_API_KEY") {
        options.llm_client  =Some("gemini".to_string());
        options.llm_model = Some("gemini-2.0-flash".to_string());
    };

    let result = converter.convert("tests/test_files/test.jpg", Some(options));
    assert!(result.is_some());
}