use backend::FrontmatterPreprocessor;

fn main() {
    // capture args from env
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    // Check if the first argument is "supports" and the second argument exists
    if args.len() > 2 && args[1] == "supports" {
        // Check if the preprocessor supports the specified renderer
        match args[2].as_str() {
            "html" => {
                println!("received html request in args :D - returning okay");
                // Supports HTML renderer
                std::process::exit(0)
            }
            // untested
            _ => std::process::exit(1), // Does not support the specified renderer
        }
    } else {
        // Normal operation, not checking for renderer support
        let backend = FrontmatterPreprocessor::default();
        if let Err(e) = backend.handle_preprocessing() {
            eprintln!("Error processing frontmatter: {:?}", e);
            std::process::exit(1);
        }
    }
}
