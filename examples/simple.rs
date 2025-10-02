use novel_lang::parser::file;

fn main() {
    let input = r#"Hello
There
== Scene 01 ==
Hello there!"#;

    let f = file(input);

    dbg!(f);
}
