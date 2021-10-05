use token::TokenStream;

mod dfa;
mod graph;
mod lex;
mod regular;
mod token;
mod word;
mod pre_work;
mod macros;

fn main() {
    let mut args = std::env::args().skip(1);
    let file = &std::fs::read_to_string(args.next().unwrap()).unwrap();
    let token_stream: TokenStream = file.into();
    for i in token_stream {
        let x = i.unwrap();
        println!("{:?}", x);
    }
}
