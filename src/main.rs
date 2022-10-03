use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use katex_renderer::render_katex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let markdown = fs::read_to_string(args[1].clone())?;

    let preprocessed_markdown = render_katex(markdown)?;

    let mut output = File::create(args[1].clone()).expect("Unable to open file");
    write!(output, "{}", preprocessed_markdown).expect("Unable to write");

    Ok(())
}
