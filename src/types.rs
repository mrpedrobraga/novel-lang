use std::collections::HashMap;

pub type Identifier = String;
pub type TimelinePoint = String;

#[derive(Debug, PartialEq, Clone)]
pub struct File {
    pub frontmatter: Option<HashMap<String, Value>>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    RichText(RichText),
    List(Vec<Value>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scene {
    pub name: Option<String>,
    pub meta: HashMap<Identifier, Value>,
    pub items: Vec<SceneItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SceneItem {
    ActionBlock(RichText),
    DialogueBlock {
        speaker: Reference,
        block: Vec<RichText>,
    },
    SpoilerBlock(RichText),
    TaggedAction(String, RichText),
    Cont(Reference),
    If(RichText, Vec<SceneItem>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Reference {
    pub referent: String,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RichText(pub Vec<RichTextPart>);

impl RichText {
    pub fn merge(self, other: RichText) -> Self {
        RichText([self.0, other.0].concat())
    }

    pub fn is_parenthetical(&self) -> bool {
        if let Some(RichTextPart::Text(text)) = self.0.first() {
            if text.starts_with("(") {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RichTextPart {
    Text(String),
    Reference(Reference),
    FormattedSection(FormattingTag, RichText),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FormattingTag {
    Bold,
    Italic,
    BoldItalic,
    Underline,
    Strikethrough,
    Other(String),
}
