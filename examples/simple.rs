use novel_lang::parser::file;

fn main() {
    let input = include_str!("./test.nov");

    let f = file(input);

    dbg!(f);
}
