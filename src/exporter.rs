use hypertext::prelude::*;
use {
    crate::types::{File, RichText, RichTextPart, Scene, SceneItem},
    hypertext::Raw,
};

pub fn export_html(file: &File) -> String {
    maud! {
        html {
            head {
                title { (file.title) }
                (Raw::dangerously_create("<link rel=\"stylesheet\" href=\"style.css\">"))
            }
        }
        body {
            main {
                div class="header" {
                    div class="header-title" { (file.title) }
                    div class="header-subtitle" { (file.subtitle) }
                }
                @for (idx, scene) in file.scenes.iter().enumerate() {
                    (r_scene(scene, idx))
                }
            }
        }
    }
    .render()
    .into_inner()
}

fn r_scene(scene: &Scene, idx: usize) -> impl Renderable {
    maud! {
        div class="scene" {
            div class="scene-name" {
                (
                    format!("{}. {}",
                    idx + 1,
                    scene.name.clone().unwrap_or_else(|| format!("Scene {}", idx)))
                )
            }
            div class="scene-items" {
                @for item in scene.items.iter() {
                    (r_item(item))
                }
            }
        }
    }
}

fn r_item(scene_item: &SceneItem) -> impl Renderable {
    maud! {
        @match scene_item {
            SceneItem::ActionLine(rich_text) =>
                div class="scene-item-action-line" {
                    (r_rich_text(rich_text))
                }
            SceneItem::NewCurrentSpeaker(reference) => a class="scene-item-new-current-speaker" href="#" {(reference.referent)},
            SceneItem::Dialogue(rich_text) => div class="scene-item-dialogue" {(r_rich_text(rich_text))},
            SceneItem::Comment(rich_text) => div class="scene-item-comment" {(r_rich_text(rich_text))},
            SceneItem::TaggedAction(tag, rich_text) => div class=(format!("scene-item-tagged-action tag-{}", tag.to_lowercase())) {
                div class="scene-item-tagged-action-tag" {(tag)}
                div class="scene-item-tagged-action-content" {(r_rich_text(rich_text))}
            },
            SceneItem::Cont(_) => div class="todo" {},
            SceneItem::If(_, _) => div class="todo" {},
        }
    }
}

fn r_rich_text(rich_text: &RichText) -> impl Renderable {
    maud! {
        span class="rich-text" {
            @for part in rich_text.0.iter() {
                (r_rich_text_part(part))
            }
        }
    }
}

fn r_rich_text_part(rich_text_part: &RichTextPart) -> impl Renderable {
    maud! {
        @match rich_text_part {
            RichTextPart::Text(text) => (text),
            RichTextPart::Reference(_) => div class="todo" {},
            RichTextPart::FormattedSection(formatting_tag, rich_text) =>
                @match formatting_tag {
                    crate::types::FormattingTag::Bold => {
                        strong { (r_rich_text(rich_text)) }
                    }
                    crate::types::FormattingTag::Italic => {
                        em { (r_rich_text(rich_text)) }
                    }
                    crate::types::FormattingTag::BoldItalic => {
                        strong { em { (r_rich_text(rich_text)) } }
                    }
                    crate::types::FormattingTag::Underline => {
                        u { (r_rich_text(rich_text)) }
                    }
                    crate::types::FormattingTag::Strikethrough => {
                        del { (r_rich_text(rich_text)) }
                    }
                    crate::types::FormattingTag::Other(_) => div class="todo" {},
                },
        }
    }
}
