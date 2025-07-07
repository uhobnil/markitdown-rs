use crate::error::MarkitdownError;
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use quick_xml::{events::Event, reader::Reader};
use std::fs;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub struct PptxConverter;

impl DocumentConverter for PptxConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".pptx" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .pptx file, got {}", ext)
                    ));
                }
            }
        }

        fs::metadata(local_path)?;

        let data = fs::read(local_path)?;
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)?;

        let mut markdown = String::new();
        let mut slide = 1;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| MarkitdownError::Zip(format!("Failed to access file in ZIP archive: {}", e)))?;
            if file.name().starts_with("ppt/slides/") && file.name().ends_with(".xml") {
                markdown.push_str(&format!("<!-- Slide number: {} -->\n\n", slide));
                slide += 1;
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| MarkitdownError::ParseError(format!("Failed to read slide content: {}", e)))?;

                let mut reader = Reader::from_str(&content);

                let mut buf = Vec::new();
                let mut skip_buf = Vec::new();
                let mut count = 0;
                #[derive(Debug, Clone)]
                struct TableStat {
                    index: u8,
                    rows: Vec<Vec<String>>,
                }
                loop {
                    let mut found_tables = Vec::new();
                    match reader.read_event_into(&mut buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                        Event::Start(element) => {
                            if let b"p:txBody" = element.name().as_ref() {
                                let mut text_buf = Vec::new();
                                loop {
                                    text_buf.clear();
                                    match reader.read_event_into(&mut text_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                        Event::Start(element) => match element.name().as_ref() {
                                            b"a:t" => loop {
                                                println!("slide: {:?}", slide);
                                                let mut tc_buf = Vec::new();
                                                match reader.read_event_into(&mut tc_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                                    Event::Text(text) => {
                                                        println!("text: {:?}", text);
                                                        markdown
                                                            .push_str(&text.unescape().map_err(|e| MarkitdownError::ParseError(format!("Failed to unescape text: {}", e)))?);
                                                    }
                                                    Event::End(element) => {
                                                        if element.name().as_ref() == b"a:t" {
                                                            break;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                                tc_buf.clear();
                                            },
                                            _ => {}
                                        },
                                        Event::End(element) => {
                                            if element.name().as_ref() == b"p:txBody" {
                                                markdown.push_str("\n\n");
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            if let b"a:tbl" = element.name().as_ref() {
                                count += 1;
                                let mut stats = TableStat {
                                    index: count,
                                    rows: vec![],
                                };
                                // must define stateful variables
                                // outside the nested loop else they are overwritten
                                let mut row_index = 0;
                                loop {
                                    skip_buf.clear();
                                    match reader.read_event_into(&mut skip_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                        Event::Start(element) => match element.name().as_ref() {
                                            b"a:tr" => {
                                                stats.rows.push(vec![]);
                                                row_index = stats.rows.len() - 1;
                                            }
                                            b"a:tc" => loop {
                                                let mut tc_buf = Vec::new();
                                                match reader.read_event_into(&mut tc_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                                    Event::Text(text) => {
                                                        stats.rows[row_index].push(
                                                            text.unescape().map_err(|e| MarkitdownError::ParseError(format!("Failed to unescape text: {}", e)))?.to_string(),
                                                        );
                                                    }
                                                    Event::End(_) => break,
                                                    _ => {}
                                                }
                                                tc_buf.clear();
                                            },
                                            _ => {}
                                        },
                                        Event::End(element) => {
                                            if element.name().as_ref() == b"a:tbl" {
                                                found_tables.push(stats);
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                found_tables.iter().for_each(|t| {
                                    markdown.push_str("|");
                                    t.rows[0].iter().for_each(|cell| {
                                        markdown.push_str(&format!(" {} |", cell));
                                    });
                                    markdown.push_str("\n|");

                                    t.rows[0].iter().for_each(|_| {
                                        markdown.push_str("---|");
                                    });
                                    markdown.push_str("\n");

                                    t.rows.iter().skip(1).for_each(|r| {
                                        markdown.push_str("|");
                                        r.iter().for_each(|c| {
                                            markdown.push_str(&format!(" {} |", c));
                                        });
                                        markdown.push_str("\n");
                                    });
                                    markdown.push_str("\n");
                                })
                            }
                        }
                        Event::Eof => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }

    fn convert_bytes(
        &self,
        bytes: &[u8],
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ext != ".pptx" {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .pptx file, got {}", ext)
                    ));
                }
            }
        }

        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)?;

        let mut markdown = String::new();
        let mut slide = 1;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| MarkitdownError::Zip(format!("Failed to access file in ZIP archive: {}", e)))?;
            if file.name().starts_with("ppt/slides/") && file.name().ends_with(".xml") {
                markdown.push_str(&format!("<!-- Slide number: {} -->\n\n", slide));
                slide += 1;
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| MarkitdownError::ParseError(format!("Failed to read slide content: {}", e)))?;

                let mut reader = Reader::from_str(&content);

                let mut buf = Vec::new();
                let mut skip_buf = Vec::new();
                let mut count = 0;
                #[derive(Debug, Clone)]
                struct TableStat {
                    index: u8,
                    rows: Vec<Vec<String>>,
                }
                loop {
                    let mut found_tables = Vec::new();
                    match reader.read_event_into(&mut buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                        Event::Start(element) => {
                            if let b"p:txBody" = element.name().as_ref() {
                                let mut text_buf = Vec::new();
                                loop {
                                    text_buf.clear();
                                    match reader.read_event_into(&mut text_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                        Event::Start(element) => {
                                            if element.name().as_ref() == b"a:t" {
                                                loop {
                                                    println!("slide: {:?}", slide);
                                                    let mut tc_buf = Vec::new();
                                                    match reader
                                                        .read_event_into(&mut tc_buf)
                                                        .unwrap()
                                                    {
                                                        Event::Text(text) => {
                                                            println!("text: {:?}", text);
                                                            markdown.push_str(
                                                                &text.unescape().unwrap(),
                                                            );
                                                        }
                                                        Event::End(element) => {
                                                            if element.name().as_ref() == b"a:t" {
                                                                break;
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                    tc_buf.clear();
                                                }
                                            }
                                        }
                                        Event::End(element) => {
                                            if element.name().as_ref() == b"p:txBody" {
                                                markdown.push_str("\n\n");
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            if let b"a:tbl" = element.name().as_ref() {
                                count += 1;
                                let mut stats = TableStat {
                                    index: count,
                                    rows: vec![],
                                };
                                // must define stateful variables
                                // outside the nested loop else they are overwritten
                                let mut row_index = 0;
                                loop {
                                    skip_buf.clear();
                                    match reader.read_event_into(&mut skip_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                        Event::Start(element) => match element.name().as_ref() {
                                            b"a:tr" => {
                                                stats.rows.push(vec![]);
                                                row_index = stats.rows.len() - 1;
                                            }
                                            b"a:tc" => loop {
                                                let mut tc_buf = Vec::new();
                                                match reader.read_event_into(&mut tc_buf).map_err(|e| MarkitdownError::ParseError(format!("Failed to read XML event: {}", e)))? {
                                                    Event::Text(text) => {
                                                        stats.rows[row_index].push(
                                                            text.unescape().map_err(|e| MarkitdownError::ParseError(format!("Failed to unescape text: {}", e)))?.to_string(),
                                                        );
                                                    }
                                                    Event::End(_) => break,
                                                    _ => {}
                                                }
                                                tc_buf.clear();
                                            },
                                            _ => {}
                                        },
                                        Event::End(element) => {
                                            if element.name().as_ref() == b"a:tbl" {
                                                found_tables.push(stats);
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                found_tables.iter().for_each(|t| {
                                    markdown.push('|');
                                    t.rows[0].iter().for_each(|cell| {
                                        markdown.push_str(&format!(" {} |", cell));
                                    });
                                    markdown.push_str("\n|");

                                    t.rows[0].iter().for_each(|_| {
                                        markdown.push_str("---|");
                                    });
                                    markdown.push('\n');

                                    t.rows.iter().skip(1).for_each(|r| {
                                        markdown.push('|');
                                        r.iter().for_each(|c| {
                                            markdown.push_str(&format!(" {} |", c));
                                        });
                                        markdown.push('\n');
                                    });
                                    markdown.push('\n');
                                })
                            }
                        }
                        Event::Eof => break,
                        _ => {}
                    }
                    buf.clear();
                }
            }
        }

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}
