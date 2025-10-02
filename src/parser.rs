use {
    crate::types::*,
    nom::{
        IResult, Parser,
        bytes::tag,
        character::{
            anychar,
            complete::{alpha1, line_ending},
        },
        combinator::{map, opt},
        multi::{many_till, many0, many1},
        sequence::{delimited, separated_pair},
    },
    std::collections::HashMap,
};

pub fn file(input: &str) -> IResult<&str, File> {
    let parser = (opt(frontmatter), line, line, many0(scene));

    map(parser, |(frontmatter, title, subtitle, scenes)| File {
        title: Some(title),
        subtitle: Some(subtitle),
        frontmatter,
        scenes,
    })
    .parse(input)
}

fn frontmatter(input: &str) -> IResult<&str, HashMap<String, Value>> {
    let parser = delimited(
        tag("---"),
        many0(separated_pair(
            map(alpha1, |x: &str| x.to_owned()),
            tag(":"),
            value,
        )),
        tag("---"),
    );

    map(parser, |x| {
        x.into_iter().collect::<HashMap<String, Value>>()
    })
    .parse(input)
}

fn scene(input: &str) -> IResult<&str, Scene> {
    let parser = delimited(
        tag("=="),
        map(many1(anychar), |x| x.into_iter().collect::<String>()),
        tag("=="),
    );

    map(parser, |name| Scene {
        name,
        meta: Default::default(),
        items: vec![],
    })
    .parse(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    map(alpha1, |x: &str| {
        Value::RichText(RichText(vec![RichTextPart::Text(x.to_owned())]))
    })
    .parse(input)
}

fn line(input: &str) -> IResult<&str, String> {
    map(many_till(anychar, line_ending), |x| {
        x.0.into_iter().collect::<String>()
    })
    .parse(input)
}
