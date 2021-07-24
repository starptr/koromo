use pulldown_cmark::{Event, Options, Parser, html};

fn main() {
    let md_input: &str = "~~cross~~";
    let parser = Parser::new_ext(md_input, Options::all()).map(|event| match event {
        Event::Text(text) => Event::Text(text.replace("~~", "tilde").into()),
        _ => event,
    });

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    html::write_html(&mut handle, parser).unwrap();
}
