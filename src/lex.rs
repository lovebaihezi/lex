#[macro_export]
macro_rules! lexer {
    {$mod_name: ident,
        $x:tt
    } => {
        mod $mod_name {
            
        }
    };
}

#[cfg(test)]
mod lex {
    fn test_expand() {
        lexer! {
            lq,
            {
                "[0-9]+" => {
                    println("?")
                }
            }
        }
    }
}
