use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use csv::ReaderBuilder;
use std::fs::File;

pub struct CsvConverter;

impl DocumentConverter for CsvConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".csv" {
                    return None;
                }
            }
        }

        let mut markdown = String::new();
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(local_path).unwrap());
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let rc: String = record
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&str>>()
                        .join(",");
                    markdown.push_str(&rc);
                    markdown.push_str("\n");
                }
                Err(err) => {
                    markdown.push_str(&format!("{:?}\n", err));
                }
            }
        }
        return Some(DocumentConverterResult {
            title: None,
            text_content: markdown,
        });
    }
}
