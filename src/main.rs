use lex::token::TokenStream;

fn main() {
    let args = std::env::args();
    let arg = args.skip(1).next().unwrap();
    let stream = if arg.starts_with('#') {
        arg.trim_start_matches('#').to_string()
    } else {
        std::fs::read_to_string(arg).unwrap()
    };
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open("src/word.rs")
        .unwrap();
    let mut token = TokenStream::from(&stream);
    println!("{:?}", &token);
    let first = token.next();
    println!("{:?}", first);
}
