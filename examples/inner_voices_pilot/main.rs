use indexmap::IndexSet;
use novel_lang::{
    exporter::export_rich_text,
    parser,
    types::{File, Value},
};

fn main() {
    let file = read_file("./examples/inner_voices_pilot/pilot.nov").unwrap();

    list_all_scenes(&file);
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

        println!("-- {name} --\n{summary}\n");
    }
}

pub fn list_all(file: &File, tag: &'static str) {
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
