use crate::error::MarkitdownError;
use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use feed_rs::parser;
use html2md::parse_html;
use std::fs::File;
use std::io::BufReader;

pub struct RssConverter;

impl DocumentConverter for RssConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Result<DocumentConverterResult, MarkitdownError> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ![".rss", ".xml", ".atom"].contains(&ext.as_str()) {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .rss, .xml, or .atom file, got {}", ext)
                    ));
                }
            }
        }

        let file = File::open(local_path)?;
        let feed = parser::parse(BufReader::new(file))
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to parse feed: {}", e)))?;

        let mut markdown = String::new();

        if feed.feed_type == feed_rs::model::FeedType::Atom {
            markdown = parse_atom_type(feed);
        } else if feed.feed_type == feed_rs::model::FeedType::RSS2 {
            markdown = parse_rss_type(feed);
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
                if ![".rss", ".xml", ".atom"].contains(&ext.as_str()) {
                    return Err(MarkitdownError::InvalidFile(
                        format!("Expected .rss, .xml, or .atom file, got {}", ext)
                    ));
                }
            }
        }

        let feed = parser::parse(BufReader::new(bytes))
            .map_err(|e| MarkitdownError::ParseError(format!("Failed to parse feed: {}", e)))?;

        let mut markdown = String::new();

        if feed.feed_type == feed_rs::model::FeedType::Atom {
            markdown = parse_atom_type(feed);
        } else if feed.feed_type == feed_rs::model::FeedType::RSS2 {
            markdown = parse_rss_type(feed);
        }

        Ok(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}

fn parse_atom_type(feed: feed_rs::model::Feed) -> String {
    let mut markdown = String::new();
    if let Some(title) = &feed.title {
        markdown.push_str(&format!("# {}\n", title.content));
    }

    feed.entries.iter().for_each(|entry| {
        if let Some(title) = &entry.title {
            markdown.push_str(&format!("\n## {}\n", title.content));
        }

        if let Some(updated) = &entry.updated {
            markdown.push_str(&format!("Updated on:  {}\n\n", updated));
        }

        if let Some(content) = &entry.content {
            if let Some(body) = &content.body {
                markdown.push_str(&parse_html(body));
                markdown.push_str("\n");
            }
        }
    });
    markdown
}

fn parse_rss_type(feed: feed_rs::model::Feed) -> String {
    let mut markdown = String::new();
    if let Some(title) = &feed.title {
        markdown.push_str(&format!("# {}\n", title.content));
    }

    if let Some(description) = &feed.description {
        markdown.push_str(&description.content);
        markdown.push_str("\n");
    }

    feed.entries.iter().for_each(|entry| {
        if let Some(title) = &entry.title {
            markdown.push_str(&format!("\n## {}\n", title.content));
        }

        if let Some(published) = &entry.published {
            markdown.push_str(&format!("Published on:  {}\n\n", published));
        }

        if let Some(summary) = &entry.summary {
            markdown.push_str(&parse_html(&summary.content));
            markdown.push_str("\n");
        }
    });
    markdown
}

// fn parse_rss_use_rss_crate(local_path: &str) -> String {
//     let file = File::open(local_path).unwrap();
//     let channel = Channel::read_from(BufReader::new(file)).unwrap();

//     let mut markdown = String::new();

//     if !channel.title().is_empty()  {
//         markdown.push_str(&format!("# {}\n", channel.title()));
//     }

//     if channel.pub_date().is_some() {
//         markdown.push_str(channel.pub_date().unwrap());
//         markdown.push_str("\n");
//     }

//     if !channel.description().is_empty() {
//         markdown.push_str(channel.description());
//         markdown.push_str("\n");
//     }

//     for item in channel.items() {
//         if item.title().is_some() {
//             markdown.push_str(&format!("\n## {}\n", item.title().unwrap()));
//         }

//         if item.pub_date().is_some() {
//             markdown.push_str(&format!("Published on:  {}\n\n", item.pub_date().unwrap()));
//         }

//         if item.description().is_some() {
//             markdown.push_str(&parse_html(item.description().unwrap()));
//             markdown.push_str("\n");
//         }
//     }

//     use std::io::Write;
//     let mut file = std::fs::File::create("test1.md").unwrap();
//     file.write_all(format!("{:#?}", channel).as_bytes()).unwrap();

//     markdown
// }
