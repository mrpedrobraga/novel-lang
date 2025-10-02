use {
    crate::types::*,
    nom::{
        IResult, Parser,
        branch::alt,
        bytes::{complete::take_until1, tag},
        character::{
            anychar,
            complete::{alphanumeric1, line_ending, multispace0, newline, none_of, space0},
        },
        combinator::{complete, eof, map, opt},
        multi::{many_till, many1, separated_list0, separated_list1},
        sequence::{delimited, preceded},
    },
    std::collections::HashMap,
};

pub fn file(input: &str) -> IResult<&str, File> {
    let parser = (
        opt(frontmatter),
        opt(line),
        opt(line),
        separated_list0(
            multispace0,
            complete(delimited(multispace0, scene, multispace0)),
        ),
    );

    map(parser, |(frontmatter, title, subtitle, scenes)| File {
        title,
        subtitle,
        frontmatter,
        scenes,
    })
    .parse(input)
}

fn frontmatter(input: &str) -> IResult<&str, HashMap<String, Value>> {
    delimited((tag("---"), newline), key_value_list, (newline, tag("---"))).parse(input)
}

fn key_value_list(input: &str) -> IResult<&str, HashMap<String, Value>> {
    let kv_entry = map(
        (identifier, space0, tag(":"), space0, value),
        |(id, _, _, _, value)| (id, value),
    );

    map(separated_list0(newline, kv_entry), |entries| {
        entries.into_iter().collect::<HashMap<_, _>>()
    })
    .parse(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((map(rich_text, Value::RichText),)).parse(input)
}

fn scene(input: &str) -> IResult<&str, Scene> {
    let header = delimited(
        tag("=="),
        opt(map(take_until1("=="), |name: &str| name.trim().to_owned())),
        (tag("=="), line_ending),
    );

    let meta = key_value_list;

    let items = separated_list1((newline, multispace0), scene_item);

    let parser = (header, meta, multispace0, items);

    map(parser, |(name, meta, _, items)| Scene { name, meta, items }).parse(input)
}

fn scene_item(input: &str) -> IResult<&str, SceneItem> {
    alt((
        comment,
        tagged_action_line,
        dialogue,
        new_current_speaker,
        action_line,
    ))
    .parse(input)
}

fn action_line(input: &str) -> IResult<&str, SceneItem> {
    map(many1(rich_text), |rich_texts| {
        SceneItem::ActionLine(rich_texts.into_iter().reduce(RichText::merge).unwrap())
    })
    .parse(input)
}

fn tagged_action_line(input: &str) -> IResult<&str, SceneItem> {
    map(
        (
            delimited(space0, tag(">"), space0),
            identifier,
            delimited(space0, tag(":"), space0),
            rich_text,
        ),
        |(_, tag, _, rich_text)| SceneItem::TaggedAction(tag, rich_text),
    )
    .parse(input)
}

fn comment(input: &str) -> IResult<&str, SceneItem> {
    map(
        (delimited(space0, tag("//"), space0), rich_text),
        |(_, rich_text)| SceneItem::Comment(rich_text),
    )
    .parse(input)
}

fn dialogue(input: &str) -> IResult<&str, SceneItem> {
    map(
        (delimited(space0, tag("-"), space0), rich_text),
        |(_, rich_text)| SceneItem::Dialogue(rich_text),
    )
    .parse(input)
}

fn rich_text(input: &str) -> IResult<&str, RichText> {
    let parser = many1(complete(alt((
        rich_text_part_reference,
        rich_text_part_text,
    ))));
    map(parser, RichText).parse(input)
}

fn rich_text_part_reference(input: &str) -> IResult<&str, RichTextPart> {
    let parser = delimited(tag("["), reference, tag("]"));
    map(parser, RichTextPart::Reference).parse(input)
}

fn rich_text_part_text(input: &str) -> IResult<&str, RichTextPart> {
    let parser = many1(none_of("\r\n=[]"));
    map(parser, |text| {
        RichTextPart::Text(text.iter().collect::<String>())
    })
    .parse(input)
}

fn new_current_speaker(input: &str) -> IResult<&str, SceneItem> {
    let parser = delimited(tag("["), reference, tag("]"));
    map(parser, SceneItem::NewCurrentSpeaker).parse(input)
}

fn reference(input: &str) -> IResult<&str, Reference> {
    let referent = map(many1(none_of("\r\n=|]")), |x| x.iter().collect::<String>());
    let alias = map(many1(none_of("\r\n=]")), |x| x.iter().collect::<String>());

    map(
        (referent, opt(preceded(tag("|"), alias))),
        |(referent, alias)| Reference { referent, alias },
    )
    .parse(input)
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, &str::to_string).parse(input)
}

fn line(input: &str) -> IResult<&str, String> {
    map(many_till(anychar, alt((line_ending, eof))), |x| {
        x.0.into_iter().collect::<String>()
    })
    .parse(input)
}
