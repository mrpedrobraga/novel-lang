use novel_lang::{exporter::export_rich_text, parser, types::File};

fn main() {
    let file = read_file("./examples/inner_voices_pilot/pilot.nov").unwrap();
    let items = file.scenes.iter().flat_map(|s| s.items.iter());
    for item in items {
        if let novel_lang::types::SceneItem::TaggedAction(tag, rich_text) = item
            && tag == "BGM"
        {
            println!(
                "- {}",
                export_rich_text(rich_text)
                    .replace("<span class=\"rich-text\">", "")
                    .replace("</span>", "")
            )
        }
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
