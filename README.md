# markitdown-rs

markitdown-rs is a Rust library designed to facilitate the conversion of various document formats into markdown text. It is a Rust implementation of the original [markitdown](https://github.com/microsoft/markitdown) Python library.

## Features

It supports:

- [x] Excel(.xlsx)
- [x] Word(.docx)
- [x] PowerPoint
- [x] PDF
- [x] Images
- [ ] Audio
- [x] HTML
- [x] CSV(UTF-8)
- [x] Text-based formats (.xml, .rss, .atom)
- [x] ZIP

## Usage

### Command-Line

#### Installation

```
cargo install markitdown
```

#### Convert a File

```
markitdown path-to-file.pdf
```

Or use -o to specify the output file:

```
markitdown path-to-file.pdf -o document.md
```

### Rust API

#### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
markitdown = "0.1.10"
```

#### Initialize MarkItDown

```rust
use markitdown::MarkItDown;

let mut md = MarkItDown::new();
```

#### Convert a File

```rust
use markitdown::{ConversionOptions, DocumentConverterResult, MarkItDown};

// Basic conversion - file type is auto-detected
let result = md.convert("path/to/file.xlsx", None)?;

// Or explicitly specify options
let options = ConversionOptions {
    file_extension: Some(".xlsx".to_string()),
    url: None,
    llm_client: None,
    llm_model: None,
};

let result = md.convert("path/to/file.xlsx", Some(options))?;

// To use Large Language Models for image descriptions
let options = ConversionOptions {
    file_extension: Some(".jpg".to_string()),
    url: None,
    llm_client: Some("gemini".to_string()),
    llm_model: Some("gemini-2.0-flash".to_string()),
};

let result = md.convert("path/to/file.jpg", Some(options))?;

if let Some(conversion_result) = result {
    println!("Converted Text: {}", conversion_result.text_content);
} else {
    println!("Conversion failed or unsupported file type.");
}
```

#### Convert from Bytes

```rust
use markitdown::{ConversionOptions, MarkItDown};

let file_bytes = std::fs::read("path/to/file.pdf")?;

// Auto-detect file type from bytes
let result = md.convert_bytes(&file_bytes, None)?;

// Or specify options explicitly
let options = ConversionOptions {
    file_extension: Some(".pdf".to_string()),
    url: None,
    llm_client: None,
    llm_model: None,
};

let result = md.convert_bytes(&file_bytes, Some(options))?;

if let Some(conversion_result) = result {
    println!("Converted Text: {}", conversion_result.text_content);
}
```

#### Register a Custom Converter

You can extend MarkItDown by implementing the `DocumentConverter` trait for your custom converters and registering them:

```rust
use markitdown::{DocumentConverter, DocumentConverterResult, ConversionOptions, MarkItDown};
use markitdown::error::MarkitdownError;

struct MyCustomConverter;

impl DocumentConverter for MyCustomConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        // Implement file conversion logic
        todo!()
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        // Implement bytes conversion logic
        todo!()
    }
}

let mut md = MarkItDown::new();
md.register_converter(Box::new(MyCustomConverter));
```

## License

MarkItDown is licensed under the MIT License. See `LICENSE` for more details.
