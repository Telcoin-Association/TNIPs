use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use pulldown_cmark::{CowStr, Event};
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write as _};

#[derive(Default)]
pub struct FrontmatterPreprocessor;

impl FrontmatterPreprocessor {
    /// Preprocess book content.
    ///
    /// This method calls the impl `run` method for [Self] to edit content
    /// and return the processed [Book] to stdout.
    pub fn handle_preprocessing(&self) -> Result<(), Error> {
        let mut log_file = File::create("handle_log.txt")?; // Log file to write debug information
        let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

        writeln!(log_file, "parsed input :D")?;
        let book_version = Version::parse(&ctx.mdbook_version)?;
        let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

        if !version_req.matches(&book_version) {
            eprintln!(
                "Warning: The {} plugin was built against version {} of mdbook, \
                 but we're being called from version {}",
                self.name(),
                mdbook::MDBOOK_VERSION,
                ctx.mdbook_version
            );
        }

        writeln!(log_file, "running preprocess...")?;
        let processed_book = self.run(&ctx, book)?;

        writeln!(log_file, "processed_book:\n{:?}", processed_book)?;
        serde_json::to_writer(io::stdout(), &processed_book)?;

        Ok(())
    }
}

impl Preprocessor for FrontmatterPreprocessor {
    fn name(&self) -> &str {
        "frontmatter-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut log_file = File::create("run_log.txt")?; // Log file to write debug information

        // TODO: could be problematic if users assume "---" is valid since markdown?
        // // default frontmatter delimiter is "+++"
        // let mut frontmatter_delimiter = "+++";

        // let frontmatter_symbol = if let Some(toml::Value::String(val)) =
        //     ctx.config.get("preprocessor.frontmatter.symbol")
        // {
        //     let _ = writeln!(log_file, "frontmatter symbol from toml:", val);
        // };

        let frontmatter_delimiter = CowStr::Borrowed("+++");

        // loop through each book item to parse chapters
        book.for_each_mut(|item| {
            let _ = writeln!(log_file, "each: {:?}", item);
            // only parse chapters
            if let BookItem::Chapter(chapter) = item {
                // flag for capturing frontmatter
                let mut capture = false;
                let mut frontmatter_collection = vec![];
                let mut formatted_content = String::new();
                let mut start_index = None;
                let mut end_index = 0;

                let parser = pulldown_cmark::Parser::new(&chapter.content);

                // loop through parsed chapter for find frontmatter based on delimiter
                for (idx, event) in parser.enumerate() {
                    let _ = writeln!(log_file, "event: {:?}", event);
                    match event {
                        // handle delimiter
                        Event::Text(ref text) if text == &frontmatter_delimiter => {
                            // first time seeing delimiter, this is false
                            if capture {
                                // End capturing, process the captured text
                                let frontmatter = parse_frontmatter(&frontmatter_collection);
                                let html_table = create_html_table(&frontmatter);
                                formatted_content.push_str(&html_table);
                                frontmatter_collection.clear();
                                end_index = idx;
                            } else {
                                // Start capturing
                                start_index = Some(idx);
                            }
                            capture = !capture;
                        }
                        // capture content within frontmatter delimiters
                        Event::Text(text) if capture => {
                            frontmatter_collection.push(text.to_string())
                        }
                        _ => {
                            if !capture {
                                if let Some(event_text) = event_text(&event) {
                                    formatted_content.push_str(event_text);
                                }
                            }
                        }
                    }
                }

                // Replace the original content only if markers were found
                if let Some(start) = start_index {
                    chapter.content = formatted_content;
                }
            }
        });
        // writeln!(log_file, "Received ctx: {:?}", ctx)?;
        // writeln!(log_file, "Received book: {:?}", &book)?;
        // for item in &mut book.sections {
        //     println!("item: {:#?}", item);
        //     // if let Some(content) = &mut item.content {
        //     //     println!("content: {:#?}", content);
        //     //     if let Some(frontmatter) = content.split("---").nth(1) {
        //     //         let parsed: HashMap<String, Value> =
        //     //             serde_json::from_str(frontmatter).map_err(|e| Error::msg(e.to_string()))?;

        //     //         // Now you can manipulate the content or use handlebars to template it
        //     //         // For example, adding parsed data to the content:
        //     //         content.push_str(&format!("\n<!-- Parsed Name: {:?} -->", parsed["name"]));
        //     //     }
        //     // }
        // }
        Ok(book)
    }
}

fn parse_frontmatter(frontmatter_text: &[String]) -> HashMap<String, String> {
    frontmatter_text
        .iter()
        .filter_map(|line| {
            let parts: Vec<_> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn create_html_table(frontmatter: &HashMap<String, String>) -> String {
    let mut table = String::from("<table class='preamble'>\n");
    for (key, value) in frontmatter {
        table.push_str(&format!("<tr><th>{}</td><td>{}</td></tr>\n", key, value));
    }
    table.push_str("</table>\n");
    table
}

fn event_text<'a>(event: &'a Event) -> Option<&'a str> {
    if let Event::Text(text) = event {
        Some(text)
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use crate::FrontmatterPreprocessor;

    #[test]
    fn test_frontmatter_parsed_correctly() {
        let pp = FrontmatterPreprocessor::default();
        let _ = pp.handle_preprocessing();
    }
}
