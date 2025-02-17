use markitdown::{
    image::ImageConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_image_conversion() {
    let converter = ImageConverter;
    let options = ConversionOptions {
        file_extension: Some(".jpg".to_string()),
        url: None,
        llm_client: Some("gemini".to_string()),
        llm_model: Some("gemini-2.0-flash".to_string()),
    };

    let result = converter.convert("tests/test_files/test.jpg", Some(options));
    write_to_file(&result.as_ref().unwrap().text_content);
    assert!(result.is_some());
}

fn write_to_file(content: &str) {
    use std::io::Write;
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
