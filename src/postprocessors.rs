//! A collection of officially maintained [postprocessors][crate::Postprocessor].

use super::{Context, MarkdownEvents, PostprocessorResult};
use pulldown_cmark::{Event, CowStr};
use serde_yaml::Value;

/// This postprocessor converts all soft line breaks to hard line breaks. Enabling this mimics
/// Obsidian's _'Strict line breaks'_ setting.
pub fn softbreaks_to_hardbreaks(
    _context: &mut Context,
    events: &mut MarkdownEvents,
) -> PostprocessorResult {
    for event in events.iter_mut() {
        if event == &Event::SoftBreak {
            *event = Event::HardBreak;
        }
    }
    PostprocessorResult::Continue
}


/// This postprocessor adds `div` tags with classes containing the info of the embedded
/// document. This can then be used later on.
pub fn add_embed_info(
    context: &mut Context,
    events: &mut MarkdownEvents,
) -> PostprocessorResult {
    let key = Value::String("embed_link".to_string());

    events.insert(0, Event::Text(CowStr::from("\n<div class=\"markdown-embed\">\n<div class=\"markdown-embed-content\">\n\n")));
    events.push(
        Event::Text(
            CowStr::from(
                format!(
                    "\n</div>\n<div class=\"markdown-embed-link\" style=\"display:none;\">\n{}</div>\n</div>", 
                    context.frontmatter.get(&key).unwrap().as_str().unwrap(),
                )
            )
        )
    );
    PostprocessorResult::Continue
}


pub fn flat_hierarchy (
    context: &mut Context,
    _: &mut MarkdownEvents,
) -> PostprocessorResult {
    let dest_key = Value::String("destination".to_string());
    let destination_root = context.frontmatter.get(&dest_key).unwrap().as_str().unwrap();

    let full_path = context.destination.clone();
    let path_without_root = full_path.strip_prefix(destination_root).ok().unwrap();
    let ext = path_without_root.extension().unwrap();

    let sanitized_path = path_without_root.with_extension("").to_string_lossy().replace(".", "-").replace("/", "-");
    let full_dest_path = std::path::PathBuf::from(&destination_root).join(sanitized_path).with_extension(ext);

    // println!("Full path: {}", full_dest_path.display());
    context.destination = full_dest_path;

    PostprocessorResult::Continue

}