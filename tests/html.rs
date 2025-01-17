use markitdown::{
    html::HtmlConverter,
    model::{ConversionOptions, DocumentConverter},
};

#[test]
fn test_html_conversion() {
    let converter = HtmlConverter;
    let options = ConversionOptions {
        file_extension: Some(".html".to_string()),
        url: None,
    };

    let result = converter.convert("tests/test_files/test_blog.html", Some(options));
    assert!(result.is_some());
}
