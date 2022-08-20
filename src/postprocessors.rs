//! A collection of officially maintained [postprocessors][crate::Postprocessor].
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use std::path::Path;
use std::fs;

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

    events.insert(0, Event::Text(CowStr::from(" <div class=\"markdown-embed\">\n<div class=\"markdown-embed-content\">\n")));
    events.push(
        Event::Text(
            CowStr::from(
                format!(
                    " </div>\n<div class=\"markdown-embed-link\" style=\"display:none;\">\n{}</div>\n</div>", 
                    context.frontmatter.get(&key).unwrap().as_str().unwrap(),
                )
            )
        )
    );
    PostprocessorResult::Continue
}

pub fn hugo_frontmatter(
    context: &mut Context,
    _events: &mut MarkdownEvents,
) -> PostprocessorResult {

    // Title
    let title_key = Value::String("title".to_string());
    let title_value = context.frontmatter.get(&title_key);

    if title_value.is_none() || title_value.unwrap().is_null() {
        context.frontmatter.remove(&title_key);
        let new_title_value = Value::String(infer_note_title_from_path(&context.current_file()).to_string());
        context.frontmatter.insert(title_key, new_title_value);
    }

    // Remove alias/aliases as they have a different functionality in Hugo
    if context.frontmatter.contains_key(&Value::String("alias".to_string())) {
        context.frontmatter.remove(&Value::String("alias".to_string()));
    }
    if context.frontmatter.contains_key(&Value::String("aliases".to_string())) {
        context.frontmatter.remove(&Value::String("aliases".to_string()));
    }

    // Created Time
    let created_key = Value::String("created".to_string());
    let created_value = context.frontmatter.get(&created_key);

    let mut new_created_value = Value::String("".to_string());
    if created_value.is_none() || created_value.unwrap().is_null() {
        let metadata = fs::metadata(context.current_file()).ok().unwrap();

        if let Ok(ctime) = metadata.created() {
            let datetime: DateTime<Utc> = ctime.into();
            new_created_value = Value::String(datetime.format("%Y-%m-%d %T").to_string());
        } else {
            println!("Not supported on this platform or filesystem");
        }
    } else {
        new_created_value = created_value.unwrap().to_owned();
    }

    context.frontmatter.remove(&created_key);
    context.frontmatter.insert(Value::String("date".to_string()), new_created_value);

    // Modified Time
    let modified_key = Value::String("modified".to_string());
    let modified_value = context.frontmatter.get(&modified_key);

    let mut new_modified_value = Value::String("".to_string());
    if modified_value.is_none() || modified_value.unwrap().is_null() {
        let metadata = fs::metadata(context.current_file()).ok().unwrap();

        if let Ok(mtime) = metadata.modified() {
            let datetime: DateTime<Utc> = mtime.into();
            new_modified_value = Value::String(datetime.format("%Y-%m-%d %T").to_string());
        } else {
            println!("Not supported on this platform or filesystem");
        }
    } else {
        new_modified_value = modified_value.unwrap().to_owned();
    }

    context.frontmatter.remove(&modified_key);
    context.frontmatter.insert(Value::String("lastmod".to_string()), new_modified_value);

    // Check if `summary` is empty, and if so, remove it.
    // Modified Time
    let summary_key = Value::String("summary".to_string());
    let summary_value = context.frontmatter.get(&summary_key);

    if !summary_value.is_none() && summary_value.unwrap().is_null() {
        context.frontmatter.remove(&summary_key);
    }

    // Change `id` field to `url` field
    let id_key = Value::String("id".to_string());
    let id_value = context.frontmatter.remove(&id_key).unwrap();
    context.frontmatter.insert(Value::String("url".to_string()), id_value);

    PostprocessorResult::Continue
}



fn infer_note_title_from_path(path: &Path) -> CowStr {
    const PLACEHOLDER_TITLE: &str = "invalid-note-title";

    match path.file_stem() {
        None => CowStr::from(PLACEHOLDER_TITLE),
        Some(s) => CowStr::from(s.to_string_lossy().into_owned()),
    }
}