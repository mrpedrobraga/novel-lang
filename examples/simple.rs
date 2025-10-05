use {
    novel_lang::{exporter::export_html, parser::file},
    std::fs,
};

fn main() {
    let input = include_str!("./example.nov");

    let f = file(input);

    if let Ok((_, file)) = f {
        dbg!(&file);
        fs::write("./examples/test.nov.html", export_html(&file)).unwrap();
    } else {
        let _ = dbg!(f);
    }
}
