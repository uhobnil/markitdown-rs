use crate::llm;
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use exif::Reader;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use tokio;

pub struct ImageConverter;

impl DocumentConverter for ImageConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".jpg" {
                    return None;
                }
            }
        }

        if !fs::metadata(local_path).is_ok() {
            return None;
        }

        let file = File::open(local_path).unwrap();
        let exif = Reader::new()
            .read_from_container(&mut BufReader::new(&file))
            .unwrap();

        let mut markdown = String::new();

        for field in exif.fields() {
            markdown.push_str(&format!(
                "{}: {}\n",
                field.tag,
                field.display_value().with_unit(&exif)
            ));
        }

        if let Some(opts) = &args {
            if let Some(llm_client) = &opts.llm_client {
                if let Some(llm_model) = &opts.llm_model {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();

                    let llm_description = rt.block_on(async {
                        llm::get_llm_description(local_path, llm_client, llm_model)
                            .await
                            .unwrap()
                    });
                    markdown.push_str("\n# Description:\n");
                    markdown.push_str(&llm_description);
                }
            }
        }

        println!("markdown:{}", markdown);

        Some(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
