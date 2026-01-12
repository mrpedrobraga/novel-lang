use std::time::Duration;

use indexmap::IndexSet;
use novel_lang::{
    exporter::export_rich_text,
    parser,
    types::{File, Value},
};

fn main() {
    let file = read_file("./examples/inner_voices_pilot/pilot.nov").unwrap();

    estimate_duration(&file);
}

pub fn list_all_scenes(file: &File) {
    for (index, scene) in file.scenes.iter().enumerate() {
        let name = scene
            .name
            .clone()
            .unwrap_or_else(|| format!("Scene {index}"));
        let summary = scene
            .meta
            .get("Summary")
            .map(Value::as_string)
            .unwrap_or_else(|| "...".to_string());

        println!("{index}. {name}");
        //println!("-- {name} --\n{summary}\n");
    }
}

pub fn list_each_tag(file: &File, tag: &'static str) {
    let deduplicate = false;
    let mut set = IndexSet::new();

    let items = file.scenes.iter().flat_map(|s| s.items.iter());
    for item in items {
        if let novel_lang::types::SceneItem::TaggedAction(_tag, rich_text) = item
            && _tag == tag
        {
            let entry = format!(
                "- {}",
                export_rich_text(rich_text)
                    .replace("<span class=\"rich-text\">", "")
                    .replace("</span>", "")
            );

            if deduplicate {
                set.insert(entry);
            } else {
                println!("{}", entry);
            }
        }
    }

    if deduplicate {
        set.iter().for_each(|e| println!("{e}"));
    }
}

pub fn estimate_duration(file: &File) {
    use novel_lang::types::SceneItem::*;

    let mut scene_count = 0;
    let mut duration_seconds = 0;
    let mut dialogue_count = 0;
    let mut descriptions = 0;

    for scene in file.scenes.iter() {
        scene_count += 1;
        for item in scene.items.iter() {
            match item {
                ActionBlock(_) => {
                    duration_seconds += 2;
                    descriptions += 1;
                }
                DialogueBlock {
                    speaker: _,
                    block: _,
                } => {
                    duration_seconds += 5;
                    dialogue_count += 1
                }
                _ => duration_seconds += 0,
            }
        }
    }

    println!(
        "Duration = {};\tDialogues = {};\tDescriptions = {}\nSections = {};\t\t\tSection Duration = {}",
        format_duration(Duration::from_secs(duration_seconds)),
        dialogue_count,
        descriptions,
        scene_count,
        format_duration(Duration::from_secs(scene_count * 60 * 5))
    );
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis(); // Optional milliseconds

    format!("{:02}h{:02}m{:02}.{:03}s", hours, minutes, seconds, millis)
}

pub fn read_file<P: AsRef<std::path::Path>>(path: P) -> Result<File, FileReadError> {
    let raw = std::fs::read_to_string(&path).map_err(FileReadError::IO)?;
    let (_, file) = parser::file(&raw).map_err(|_| FileReadError::Parse)?;
    Ok(file)
}

#[derive(Debug)]
pub enum FileReadError {
    IO(std::io::Error),
    Parse,
}
