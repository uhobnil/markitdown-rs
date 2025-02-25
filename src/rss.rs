use crate::model::{ConversionOptions, DocumentConverter, DocumentConverterResult};
use std::fs::File;
use std::io::BufReader;
use html2md::parse_html;
use feed_rs::parser;

pub struct RssConverter;

impl DocumentConverter for RssConverter {
    fn convert(
        &self,
        local_path: &str,
        args: Option<ConversionOptions>,
    ) -> Option<DocumentConverterResult> {
        if let Some(opts) = &args {
            if let Some(ext) = &opts.file_extension {
                if ![".rss", ".xml", ".atom"].contains(&ext.as_str()) {
                    return None;
                }
            }
        }

        let file = File::open(local_path).unwrap();
        let feed = parser::parse(BufReader::new(file)).unwrap();
        
        let mut markdown = String::new();

        if feed.feed_type == feed_rs::model::FeedType::Atom {
            markdown = parse_atom_type(feed);
        }else if feed.feed_type == feed_rs::model::FeedType::RSS2 {
            markdown = parse_rss_type(feed);
        }

        Some(DocumentConverterResult {
            title: None,
            text_content: markdown,
        })
    }
}

fn parse_atom_type(feed: feed_rs::model::Feed) -> String {
    let mut markdown = String::new();
    if feed.title.is_some() {
        markdown.push_str(&format!("# {}\n", feed.title.unwrap().content));
    }

    feed.entries.iter().for_each(|entry| {
        if entry.title.is_some() {
            markdown.push_str(&format!("\n## {}\n", entry.title.clone().unwrap().content));
        }

        if entry.updated.is_some() {
            markdown.push_str(&format!("Updated on:  {}\n\n", entry.updated.unwrap()));
        }

        if entry.content.is_some() {
            markdown.push_str(&parse_html(&entry.content.clone().unwrap().body.unwrap()));
            markdown.push_str("\n");
        }
    });
    markdown
}

fn parse_rss_type(feed: feed_rs::model::Feed) -> String {
    let mut markdown = String::new();
    if feed.title.is_some() {
        markdown.push_str(&format!("# {}\n", feed.title.unwrap().content));
    }

    if feed.description.is_some() {
        markdown.push_str(&feed.description.unwrap().content);
        markdown.push_str("\n");
    }

    feed.entries.iter().for_each(|entry| {
        if entry.title.is_some() {
            markdown.push_str(&format!("\n## {}\n", entry.title.clone().unwrap().content));
        }

        if entry.published.is_some() {
            markdown.push_str(&format!("Published on:  {}\n\n", entry.published.unwrap()));
        }

        if entry.summary.is_some() {
            markdown.push_str(&parse_html(&entry.summary.clone().unwrap().content));
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