#[macro_export]
macro_rules! lex {
    {
        {
        $(
            $rules:expr => $(
                $run:block
            )*
        ),*$(,)*
    }} => {
        fn lexer() {}
    };
}

lex! {{
    "[0-9]+" => {
        println!("{}")
    }
}}

#[cfg(test)]
mod macros {
    #[test]
    fn lex_expand() {}
}
