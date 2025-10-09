use crate::types::{File, RichText, RichTextPart, Scene, SceneItem, Value};
use hypertext::prelude::*;

pub fn export_html(file: &File) -> String {
    maud! {
        body {
            main {
                @if let Some(frontmatter) = &file.frontmatter {
                    div class="header" {
                        @if let Some(title) = frontmatter.get("T") {
                            div class="header-title" {(r_value(title))}
                        }

                        @if let Some(subtitle) = frontmatter.get("S") {
                            div class="header-subtitle" {(r_value(subtitle))}
                        }
                    }

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
                    scene.name.clone().unwrap_or_else(|| format!("Scene {}", idx + 1)))
                )
            }
            div class="marker" id=(scene.name.as_ref().map(|name| name.replace(" ", "_"))) {}
            div class="scene-items" {
                @for item in scene.items.iter() {
                    (r_item(item))
                }
            }
        }
    }
}

fn format_contd(speaker_string: &str) -> &str {
    if speaker_string == "&" {
        return "CONT'D";
    }

    speaker_string
}

fn r_item(scene_item: &SceneItem) -> impl Renderable {
    maud! {
        @match scene_item {
            SceneItem::ActionBlock(rich_text) =>
                div class="scene-item-action-line" {
                    (r_rich_text(rich_text))
                }
                SceneItem::DialogueBlock { speaker, block } => {
                    a class="scene-item-new-current-speaker" href="#" {(format_contd(&speaker.referent))}
                    @for rich_text in block {
                        div class={
                            "scene-item-dialogue"
                            (if rich_text.is_parenthetical() {" parenthetical"} else {""})
                        } {(r_rich_text(rich_text))}
                    }
                }
            SceneItem::SpoilerBlock(rich_text) => div class="scene-item-comment" {(r_rich_text(rich_text))},
            SceneItem::TaggedAction(tag, rich_text) => div class=(format!("scene-item-tagged-action tag-{}", tag.to_lowercase())) {
                div class="scene-item-tagged-action-tag" {(tag)}
                div class="scene-item-tagged-action-content" {(r_rich_text(rich_text))}
            },
            SceneItem::Cont(_) => div class="todo" {},
            SceneItem::If(_, _) => div class="todo" {},
        }
    }
}

fn r_value(value: &Value) -> impl Renderable {
    maud! {
        @match value {
            Value::RichText(rich_text) => (r_rich_text(rich_text)),
            Value::List(values) => div class="value_list" {
                @for value in values {
                    (r_value(value))
                }
            },
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
            RichTextPart::Reference (
                reference
            ) => a class="reference" href=(reference.referent) { (reference.alias.as_ref().unwrap_or(&reference.referent)) },
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
