use std::collections::HashMap;

pub type Identifier = String;
pub type TimelinePoint = String;

pub struct File {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub frontmatter: HashMap<String, Value>,
    pub scenes: Vec<Scene>,
}

pub struct Scene {
    pub meta: HashMap<Identifier, Value>,
}

pub enum Value {
    RichText(RichText),
    List(Vec<Value>),
}

pub enum SceneItem {
    ActionLine(RichText),
    NewCurrentSpeaker(Reference),
    Dialogue(RichText),
    Comment(RichText),
    TaggedAction(String, RichText),
    Cont(Reference),
    If(RichText, Vec<SceneItem>),
}

pub struct Reference {
    pub referent: String,
}

pub struct RichText(pub Vec<RichTextPart>);

pub enum RichTextPart {
    Text(String),
    Reference(Reference),
    FormattedSection(FormattingTag, Vec<RichTextPart>),
}

pub enum FormattingTag {
    Bold,
    Italic,
    BoldItalic,
    Underline,
    Strikethrough,
    Other(String),
}
