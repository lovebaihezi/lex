use super::*;
use std::{
    convert::{TryFrom, TryInto},
    mem::swap,
};

use super::{TokenError, Tokens};

pub struct TokenStream<I>
where
    I: Iterator<Item = char>,
{
    stream: I,
    tokens: Result<Tokens, TokenError>,
    line: u32,
    col: u32,
}

impl<I> TokenStream<I>
where
    I: Iterator<Item = char>,
{
    #[inline]
    pub fn new(v: I) -> Self {
        TokenStream {
            stream: v,
            tokens: Ok(Default::default()),
            line: Default::default(),
            col: Default::default(),
        }
    }
    #[inline]
    fn calc(&mut self, tokens: &mut Result<Tokens, TokenError>) -> (u32, [u32; 2]) {
        let current_length = match &self.tokens {
            Ok(v) => match v {
                Tokens::Ident(s) => s.len() as u32,
                Tokens::KeyWords(key_word) => key_word.len(),
                Tokens::Constant(constant) => constant.len() as u32,
                Tokens::Comment(comment) => comment.len() as u32,
                Tokens::Sign(sign) => sign.len(),
                Tokens::Empty => 1,
                Tokens::BreakLine => {
                    self.line += 1;
                    0
                }
            },
            _ => 0,
        };
        let col = self.col;
        self.col += current_length;
        swap(tokens, &mut self.tokens);
        (self.line, [col, self.col])
    }
}

impl<I> Iterator for TokenStream<I>
where
    I: Iterator<Item = char>,
{
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stream.next() {
            let mut value: Result<Tokens, TokenError> = match &mut self.tokens {
                Ok(token) => match token {
                    Tokens::Ident(ident) => {
                        let key_word: Result<KeyWords, &str> = ident.as_str().try_into();
                        match key_word {
                            Ok(v) => {
                                self.tokens = Ok(v.into());
                                current.try_into()
                            }
                            Err(_) => match current {
                                '\u{0000}'..='\u{0008}' | '\u{000E}'..='\u{001F}' => {
                                    Err(TokenError::ControlCode)
                                }
                                '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+' | ',' | '-'
                                | '.' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\'
                                | ']' | '^' | '_' | '`' | '{' | '|' | '}' | '~' | '/' | '\''
                                | '"' => {
                                    Ok(Tokens::Sign(Single::try_from(current).unwrap().into()))
                                }
                                // 'F' => Ok(KeyWords::F.into()),
                                // 'Y' => Ok(KeyWords::Y.into()),
                                ' ' => Ok(Tokens::Empty),
                                '\n' => Ok(Tokens::BreakLine),
                                '\r' => continue,
                                _ => {
                                    ident.push(current);
                                    continue;
                                }
                            },
                        }
                    }
                    Tokens::KeyWords(_) => current.try_into(),
                    Tokens::Constant(constant) => match constant {
                        Constant::String(s) => match current {
                            '"' => {
                                if s.ends_with('\\') {
                                    Ok(Tokens::Empty)
                                } else {
                                    continue;
                                }
                            }
                            _ => {
                                s.push(current);
                                continue;
                            }
                        },
                        Constant::Number(s) => match current {
                            '0'..='9' => {
                                s.push(current);
                                continue;
                            }
                            '_' => continue,
                            _ => current.try_into(),
                        },
                        _ => current.try_into(),
                    },
                    Tokens::Comment(s) => {
                        if current == '\n' {
                            Ok(Tokens::BreakLine)
                        } else if current == '\r' {
                            continue;
                        } else {
                            s.push(current);
                            continue;
                        }
                    }
                    Tokens::Sign(sign) => match sign {
                        Sign::Double(Double::SlashSlash) => {
                            self.tokens = Ok(Tokens::Comment(String::new()));
                            continue;
                        }
                        v => {
                            let s: Result<Tokens, TokenError> = current.try_into();
                            match s {
                                Ok(Tokens::Sign(sign)) => match sign + *v {
                                    Some(real) => {
                                        *v = real;
                                        continue;
                                    }
                                    None => s,
                                },
                                v => v,
                            }
                        }
                    },
                    _ => current.try_into(),
                },
                Err(_) => current.try_into(),
            };
            let (line, col) = self.calc(&mut value);
            return Some(value.map(|content| Token { content, line, col }));
        }
        match self.tokens {
            Ok(Tokens::Empty) => None,
            _ => {
                let mut content = Ok(Tokens::Empty);
                let (line, col) = self.calc(&mut content);
                Some(content.map(|content| Token { content, line, col }))
            }
        }
    }

    type Item = Result<Token, TokenError>;
}
