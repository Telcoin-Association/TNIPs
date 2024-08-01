use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use pulldown_cmark::Event;
use semver::{Version, VersionReq};
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

        book.for_each_mut(|item| {
            let _ = writeln!(log_file, "each: {:?}", item);
            // only parse chapters
            if let BookItem::Chapter(chapter) = item {
                // flag for capturing frontmatter
                let mut capture = false;
                let mut frontmatter = vec![];

                let parser = pulldown_cmark::Parser::new(&chapter.content);
                for event in parser {
                    let _ = writeln!(log_file, "event: {:?}", event);
                    match event {
                        Event::Text(ref text) if text == "+++" => {
                            // flip flag
                            capture = !capture;
                            if capture {
                                // Parse the captured frontmatter text
                                let frontmatter = parse_frontmatter(&frontmatter_text);
                                let _ = writeln!(log_file, "frontmatter: {:?}", frontmatter);

                                // Create HTML table from frontmatter data
                                let html_table = create_html_table(&frontmatter);
                                chapter.content.push_str(&html_table);

                                frontmatter_text.clear();
                                continue;
                            }

                            // capture is false stop loop
                            break;
                        }
                        Event::Text(text) if capture => frontmatter_text.push(text.to_string()),
                        _ => {}
                    }
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

#[cfg(test)]
mod tests {
    use crate::FrontmatterPreprocessor;

    #[test]
    fn test_frontmatter_parsed_correctly() {
        let pp = FrontmatterPreprocessor::default();
        let _ = pp.handle_preprocessing();
    }
}
