use katex::{self, OutputType};
use std::error::Error;

fn render_inline_tex(tex: &str) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(false)
        .output_type(OutputType::Html)
        .build()
        .unwrap();
    Ok(katex::render_with_opts(tex, &opts)?)
}

fn render_block_tex(tex: &str) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(true)
        .output_type(OutputType::Html)
        .build()
        .unwrap();
    Ok(katex::render_with_opts(tex, &opts)?)
}

pub fn render_katex(tex: String) -> Result<String, Box<dyn Error>> {
    let mut md_katex = String::new();
    let mut start = 0;
    let mut end = false;
    let code_tags: Vec<usize> = tex
        .match_indices('`')
        .into_iter()
        .map(|(sz, _)| sz)
        .collect();
    let indices: Vec<(usize, &str)> = tex.match_indices("$$").into_iter().collect();
    for (index, _) in &indices {
        if code_tags.iter().filter(|v| *v < index).count() % 2 != 0 {
            continue;
        }
        if !end {
            md_katex.push_str(&tex[start..*index]);
            end = true;
            start = *index;
        } else {
            md_katex.push_str(&render_block_tex(&tex[(start + 2)..*index])?);
            end = false;
            start = *index + 2;
        }
    }
    md_katex.push_str(&tex[start..]);
    let code_tags: Vec<usize> = md_katex
        .match_indices('`')
        .into_iter()
        .map(|(sz, _)| sz)
        .collect();
    let mut start = 0;
    let mut end = false;
    let mut preprocessed_markdown = String::new();
    let indices: Vec<(usize, &str)> = md_katex.match_indices('$').into_iter().collect();
    for (index, _) in &indices {
        if code_tags.iter().filter(|v| *v < index).count() % 2 != 0 {
            continue;
        }
        if !end {
            preprocessed_markdown.push_str(&md_katex[start..*index]);
            end = true;
            start = *index;
        } else {
            preprocessed_markdown.push_str(&render_inline_tex(&md_katex[(start + 1)..*index])?);
            end = false;
            start = *index + 1;
        }
    }
    preprocessed_markdown.push_str(&md_katex[start..]);

    Ok(preprocessed_markdown)
}
