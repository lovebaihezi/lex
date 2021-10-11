use lex::token::{KeyWords, TokenStream};

fn main() {
    let mut args = std::env::args().skip(1);
    let file = &std::fs::read_to_string(args.next().unwrap()).unwrap();
    let token_stream: TokenStream = file.into();
    for i in token_stream {
        print!("{}", i.unwrap().content.to_string())
    }
    println!();
    let token_stream: TokenStream = file.into();
    for i in token_stream {
        println!("{:?}", i.unwrap().content)
    }
    KeyWords::get_all()
        .iter()
        .for_each(|v| println!("{}", v.to_string()))
}
