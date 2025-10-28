use crate::types::{File, RichText, Scene, SceneItem, Value};

pub fn play(file: File) {
    if let Some(frontmatter) = file.frontmatter {
        if let Some(title) = frontmatter.get("T") {
            play_value(title);
        }

        if let Some(subtitle) = frontmatter.get("S") {
            play_value(subtitle);
        }
    }

    let mut scene_index = 0;
    let mut current_scene = file.scenes.first();

    while let Some(scene) = &current_scene {
        play_scene(scene);
        scene_index += 1;
        current_scene = file.scenes.get(scene_index);
    }
}

pub fn wait() {
    use std::io::Read;
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}

pub fn play_scene(scene: &Scene) {
    for item in scene.items.iter() {
        play_item(item);
        wait();
    }
}

pub fn play_item(item: &SceneItem) {
    match item {
        SceneItem::ActionBlock(rich_text) => play_rich_text(rich_text),
        SceneItem::DialogueBlock { speaker, block } => {
            play_text(speaker.alias.as_ref().unwrap_or(&speaker.referent));
            for line in block.iter() {
                play_rich_text(line);
            }
        }
        SceneItem::SpoilerBlock(_) => {}
        SceneItem::TaggedAction(_, rich_text) => play_rich_text(rich_text),
        SceneItem::Cont(_) => todo!(),
        SceneItem::If(_, _) => todo!(),
    }
}

pub fn play_value(value: &Value) {
    match value {
        Value::RichText(rich_text) => play_rich_text(rich_text),
        Value::List(values) => {
            println!("[");
            for value in values {
                play_value(value);
            }
            println!("]");
        }
    }
}

pub fn play_rich_text(rich_text: &RichText) {
    for part in rich_text.0.iter() {
        match part {
            crate::types::RichTextPart::Text(text) => play_text(text),
            crate::types::RichTextPart::Reference(reference) => play_text(&reference.referent),
            crate::types::RichTextPart::FormattedSection(_, rich_text) => play_rich_text(rich_text),
        }
    }
}

pub fn play_text(text: &str) {
    println!("{}", text);
}
