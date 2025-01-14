use markitdown::{
    model::{ConversionOptions, DocumentConverter},
    pdf::PdfConverter,
};

#[test]
fn test_pdf_conversion() {
    let converter = PdfConverter;
    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
    };

    let result = converter.convert("tests/test_files/test.pdf", Some(options));
    write_to_file(&result.as_ref().unwrap().text_content);
    assert!(result.is_some());
}

fn write_to_file(content: &str) {
    use std::io::Write;
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
