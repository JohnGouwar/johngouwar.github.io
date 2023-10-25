use std::path::PathBuf;

use build_html::{Container, HtmlContainer};
use clap::Parser;

mod index;
mod publications;
mod repr;
mod html;
mod toml_index;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "content")]
    content: PathBuf,

    #[arg(long, default_value = "publications")]
    publications: PathBuf,

}

fn main() {
    let args = Cli::parse();
    let about_text = std::fs::read_to_string(args.content.join("about.md"))
        .expect("Failed to read about.md");
    let about_html = Container::default()
        .with_attributes([("class", "about")])
        .with_raw(markdown::to_html(&about_text));
    let pubs_html = publications::compile_publications(args.publications);
    let full_html = index::build_index(vec![about_html, pubs_html]);
    let index_path = "index.html";
    let _ = std::fs::write(index_path, full_html).expect("Failed to write index.html");

}
