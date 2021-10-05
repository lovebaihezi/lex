use std::{
    cell::RefCell,
    convert::{TryFrom, TryInto},
    mem::swap,
    ops::Add,
};

macro_rules! KeyWords {
    ({$($derive: path),*$(,)*}, $($id:ident),*$(,)*) => {
        #[derive($($derive),*)]
        pub enum KeyWords {
            $($id,)+
        }
        const LEN:usize = [$(KeyWords::$id,)+].len();
        const KEYWORDS_ARRAY: [KeyWords; LEN] = [$(KeyWords::$id,)+];
        const KEYWORDS_STRING: [&'static str; LEN] = [$(stringify!(KeyWords::$id),)+];
        const KEYWORDS_LENGTHS: [u32; LEN] = [$(stringify!($id).len() as u32,)+];
        impl KeyWords {
            #[inline]
            fn len(&self) -> u32 {
                unsafe { *KEYWORDS_LENGTHS.get_unchecked(*self as usize) }
            }
            #[inline]
            fn get_all<'a>() -> &'a [KeyWords] {
                &KEYWORDS_ARRAY
            }
        }
        impl<'a> TryFrom<&'a str> for KeyWords {
            type Error = &'a str;
            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                match value {
                    $(
                        stringify!($id) => Ok(Self::$id),
                    )+
                    _ => Err(value)
                }
            }
        }
        impl ToString for KeyWords {
            #[inline]
            fn to_string(&self) -> String {
                unsafe { String::from(*KEYWORDS_STRING.get_unchecked(*self as usize)) }
            }
        }
    };
}

macro_rules! Convert {
    ($e: ident) => {
        impl From<$e> for Sign {
            #[inline]
            fn from(v: $e) -> Self {
                Self::$e(v)
            }
        }
    };
    ($e: ident, $x: ident) => {
        impl From<$e> for $x {
            #[inline]
            fn from(v: $e) -> Self {
                Self::$e(v)
            }
        }
    };
}

macro_rules! SignAdd {
    {$(($v: path | $v1: path)) ,* $(,)*} => {
        impl Add for Sign {
            type Output = Option<Sign>;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(($v(v), $v1(v1)) => match v + v1 {
                        Some(v) => Some(v.into()),
                        None => None,
                    },)+
                    _ => None,
                }
            }
        }
    };
}

macro_rules! ConstArray {
    ($name: ident, [$($id: ty),*$(,)*]) => {
        const $name: [&'static str; [$(stringify!($id)),*].len()] = [$(stringify!($id),)*];
    };
}

macro_rules! Single {
    ({$($id:ident => $real:expr),*$(,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Single {
            $($id,)*
        }
        const SINGLE_ARRAY_LEN: usize = [$($real,)*].len();
        const SINGLE_ARRAY: [char; SINGLE_ARRAY_LEN] = [$($real,)*];
        const fn single_map() -> [Option<Single>; 127] {
            let mut arr = [None; 127];
            let mut index = 0u8;
            while index <= 126 {
                arr[index as usize] = match index as char {
                    $($real => Some(Single::$id),)*
                    _ => None
                };
                index += 1;
            }
            arr
        }
        const SINGLE_MAP: [Option<Single>; 127] = single_map();
        impl TryFrom<char> for Single {
            type Error = SignError;
            fn try_from(v: char) -> Result<Self, Self::Error> {
                if v > '\u{007E}' {
                    Err(SignError::NotSign)
                } else {
                    match unsafe { *SINGLE_MAP.get_unchecked(v as usize) } {
                        Some(v) => Ok(v),
                        _ => Err(SignError::NotMatch),
                    }
                }
            }
        }
        impl Into<char> for Single {
            fn into(self) -> char {
                unsafe {
                    *SINGLE_ARRAY.get_unchecked(self as usize)
                }
            }
        }
    };
}

macro_rules! Double {
    {$($left:ident => $real:expr),*$(,)*} => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Double {
            $($left,)*
        }
        const DOUBLE_LEN: usize = [$($real),*].len();
        // const DOUBLE_STR_ARRAY: [&'static str; DOUBLE_LEN] = [$($real),*];
        impl<'a> TryFrom<&'a str> for Double {
            type Error = &'a str;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                match s {
                    $($real => Ok(Double::$left),)*
                    _ => Err(s)
                }
            }
        }
        impl ToString for Double {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$left => $real,)*
                }.to_string()
            }
        }
    }
}

Single!({
    Exclamation       => '!',
    Quotation         => '"',
    Hash              => '#',
    Dollar            => '$',
    Percent           => '%',
    Ampersand         => '&',
    Apostrophe        => '\'',
    LeftParenthesis   => '(',
    RightParenthesis  => ')',
    Asterisk          => '*',
    Plus              => '+',
    Comma             => ',',
    Minus             => '-',
    Period            => '.',
    Slash             => '/',
    Colon             => ':',
    Semicolon         => ';',
    Less              => '<',
    Equal             => '=',
    Greater           => '>',
    Question          => '?',
    At                => '@',
    LeftSquareBracket => '[',
    BackSlash         => '\\',
    RightSquareBracket => ']',
    Caret              => '^',
    UnderScore         => '_',
    GraveAccent        => '`',
    LeftCurlyBracket   => '{',
    VerticalBar        => '|',
    RightCurlyBracket  => '}',
    Tilde              => '~',
});

Double! {
    EqualEqual                    => "==",
    EqualGreater                  => "=>",
    LessEqual                     => "<=",
    GreaterEqual                  => ">=",
    EqualLess                     => "=<",
    LessMinus                     => "<-",
    MinusGreater                  => "->",
    VerticalBarGreater            => "|>",
    ColonColon                    => "::",
    QuestionQuestion              => "??",
    ExclamationExclamation        => "!!",
    VerticalBarVerticalBar        => "||",
    ColonEqual                    => ":=",
    VerticalBarEqual              => "|=",
    GreaterGreater                => ">>",
    LessLess                      => "<<",
    ExclamationEqual              => "!=",
    LessVerticalBar               => "<|",
    PeriodQuestion                => ".?",
    PeriodExclamation             => ".!",
    AsteriskEqual                 => "*=",
    PlusEqual                     => "+=",
    MinusEqual                    => "-=",
    SlashEqual                    => "/=",
    HashHash                      => "##",
    AmpersandAmpersand            => "&&",
    CaretCaret                    => "^^",
    LeftCurlyBracketVerticalBar   => "{|",
    VerticalBarRightCurlyBracket  => "|}",
    LeftSquareBracketVerticalBar  => "[|",
    VerticalBarRightSquareBracket => "|]",
    PeriodPeriod                  => "..",
    SlashSlash                    => "//",
}

ConstArray!(
    NUMBER_TYPE,
    [i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,]
);

KeyWords! ({Debug, Clone, Copy},
    Back ,Move, F, Y, Yield, Async, Await, Trait,
    Implement, For, Bind, Type, Enum, Struct,
    Parallel, Cast, Tobe, Module, Where, Loop,
    While, When, Match, Macro, Public, Dynamic,
    Box, Atomic, Const, Static, Lazy, In, From,
    To, Reference, Extern, Do, Algin, Mutable,
    Block, Expression,
);

#[derive(Debug)]
pub enum Constant {
    String(String),
    Char(char),
    Number(String),
}

impl Constant {
    fn len(&self) -> u32 {
        match self {
            Constant::Char(_) => 1,
            Constant::String(s) | Constant::Number(s) => s.len() as u32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignError {
    NotSign,
    NotMatch,
}

#[derive(Debug, Clone, Copy)]
pub enum Triple {
    EqualEqualEqual,       // ===
    PeriodPeriodPeriod,    // ...
    ExclamationEqualEqual, // !==
    GreaterGreaterGreater, // >>>
}

#[derive(Debug, Clone, Copy)]
pub enum Quadruple {}

impl Add for Single {
    type Output = Option<Double>;

    /*
    Equal,Equal
    Equal,Greater
    Less,Equal
    Greater,Equal
    Equal,Less
    Less,Minus
    Minus,Greater
    VerticalBar,Greater
    Colon,Colon
    Question,Question
    Exclamation,Exclamation
    VerticalBar,VerticalBar
    Colon,Equal
    VerticalBar,Equal
    Greater,Greater
    Less,Less
    Exclamation,Equal
    Less,VerticalBar
    Period,Question
    Period,Exclamation
    Asterisk,Equal
    Plus,Equal
    Minus,Equal
    Divide,Equal
    Hash,Hash
    Ampersand,Ampersand
    Caret,Caret
    LeftCurlyBracket,VerticalBar
    VerticalBar,RightCurlyBracket
    LeftSquareBracket,VerticalBar
    VerticalBar,RightSquareBracket
    Period,Period
    */

    fn add(self, rhs: Self) -> Self::Output {
        use Double::*;
        match (self, rhs) {
            (Self::Equal, Self::Equal) => Some(EqualEqual),
            (Self::Equal, Self::Greater) => Some(EqualGreater),
            (Self::Less, Self::Equal) => Some(LessEqual),
            (Self::Greater, Self::Equal) => Some(GreaterEqual),
            (Self::Equal, Self::Less) => Some(EqualLess),
            (Self::Less, Self::Minus) => Some(LessMinus),
            (Self::Minus, Self::Greater) => Some(MinusGreater),
            (Self::VerticalBar, Self::Greater) => Some(VerticalBarGreater),
            (Self::Colon, Self::Colon) => Some(ColonColon),
            (Self::Question, Self::Question) => Some(QuestionQuestion),
            (Self::Exclamation, Self::Exclamation) => Some(ExclamationExclamation),
            (Self::VerticalBar, Self::VerticalBar) => Some(VerticalBarVerticalBar),
            (Self::Colon, Self::Equal) => Some(ColonEqual),
            (Self::VerticalBar, Self::Equal) => Some(VerticalBarEqual),
            (Self::Greater, Self::Greater) => Some(GreaterGreater),
            (Self::Less, Self::Less) => Some(LessLess),
            (Self::Exclamation, Self::Equal) => Some(ExclamationEqual),
            (Self::Less, Self::VerticalBar) => Some(LessVerticalBar),
            (Self::Period, Self::Question) => Some(PeriodQuestion),
            (Self::Period, Self::Exclamation) => Some(PeriodExclamation),
            (Self::Asterisk, Self::Equal) => Some(AsteriskEqual),
            (Self::Plus, Self::Equal) => Some(PlusEqual),
            (Self::Minus, Self::Equal) => Some(MinusEqual),
            (Self::Slash, Self::Equal) => Some(SlashEqual),
            (Self::Hash, Self::Hash) => Some(HashHash),
            (Self::Ampersand, Self::Ampersand) => Some(AmpersandAmpersand),
            (Self::Caret, Self::Caret) => Some(CaretCaret),
            (Self::LeftCurlyBracket, Self::VerticalBar) => Some(LeftCurlyBracketVerticalBar),
            (Self::VerticalBar, Self::RightCurlyBracket) => Some(VerticalBarRightCurlyBracket),
            (Self::LeftSquareBracket, Self::VerticalBar) => Some(LeftSquareBracketVerticalBar),
            (Self::VerticalBar, Self::RightSquareBracket) => Some(VerticalBarRightSquareBracket),
            (Self::Period, Self::Period) => Some(PeriodPeriod),
            (Self::Slash, Self::Slash) => Some(SlashSlash),
            _ => None,
        }
    }
}

impl Add<Double> for Single {
    type Output = Option<Triple>;

    fn add(self, rhs: Double) -> Self::Output {
        match (self, rhs) {
            (Single::Equal, Double::EqualEqual) => Some(Triple::EqualEqualEqual),
            (Single::Exclamation, Double::EqualEqual) => Some(Triple::ExclamationEqualEqual),
            (Single::Period, Double::PeriodPeriod) => Some(Triple::PeriodPeriodPeriod),
            (Single::Greater, Double::GreaterGreater) => Some(Triple::GreaterGreaterGreater),
            _ => None,
        }
    }
}

impl Add<Single> for Double {
    type Output = Option<Triple>;

    fn add(self, rhs: Single) -> Self::Output {
        rhs + self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Single(Single),
    Double(Double),
    Triple(Triple),
    Quadruple(Quadruple),
}

impl Sign {
    fn len(&self) -> u32 {
        match self {
            Self::Single(_) => 1,
            Self::Double(_) => 2,
            Self::Triple(_) => 3,
            Self::Quadruple(_) => 4,
        }
    }
}

Convert!(Single);
Convert!(Double);
Convert!(Triple);
Convert!(Quadruple);

SignAdd! {
    (Sign::Single | Sign::Single),
    (Sign::Single | Sign::Double),
    (Sign::Double | Sign::Single)
}

#[derive(Debug)]
pub enum Tokens {
    Ident(String),
    KeyWords(KeyWords),
    Constant(Constant),
    Comment(String),
    Sign(Sign),
    Empty,
    BreakLine,
}

impl ToString for Tokens {
    fn to_string(&self) -> String {
        match self {
            Tokens::Ident(s) => format!("{}", s),
            Tokens::KeyWords(s) => todo!(),
            Tokens::Constant(s) => todo!(),
            Tokens::Comment(s) => todo!(),
            Tokens::Sign(s) => match s {
                Sign::Single(s) => {
                    let x: char = (*s).into();
                    x
                }
                .to_string(),
                Sign::Double(_) => todo!(),
                Sign::Triple(_) => todo!(),
                Sign::Quadruple(_) => todo!(),
            },
            Tokens::Empty => " ".to_string(),
            Tokens::BreakLine => "\n".to_string(),
        }
    }
}

impl Default for Tokens {
    #[inline]
    fn default() -> Self {
        Self::Empty
    }
}

impl From<KeyWords> for Tokens {
    fn from(v: KeyWords) -> Self {
        Tokens::KeyWords(v)
    }
}

#[derive(Debug, Default)]
pub struct Token {
    content: Tokens,
    line: u32,
    col: [u32; 2],
}

pub struct TokenStream<'a> {
    stream: Box<dyn Iterator<Item = char> + 'a>,
    tokens: Result<Tokens, TokenError>,
    line: u32,
    col: u32,
}

#[derive(Debug)]
pub enum TokenError {
    ControlCode,
    InvalidUnicode,
    InvalidIdent,
}

impl<'a, 'b: 'a> TokenStream<'a> {
    #[inline]
    fn new(v: Box<dyn Iterator<Item = char> + 'a>) -> Self {
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

impl<'a, 'b: 'a> From<&'b str> for TokenStream<'a>
// where
// T: IntoIterator<Item = char> + std::iter::Iterator<Item = char>,
{
    #[inline]
    fn from(v: &'b str) -> Self {
        Self::new(Box::new(v.chars()))
    }
}

impl<'a, 'b: 'a> From<&'b String> for TokenStream<'a>
// where
// T: IntoIterator<Item = char> + std::iter::Iterator<Item = char>,
{
    #[inline]
    fn from(v: &'b String) -> Self {
        Self::new(Box::new(v.chars()))
    }
}

impl<'a> Iterator for TokenStream<'a> {
    fn next(self: &mut Self) -> Option<Self::Item> {
        while let Some(current) = self.stream.next() {
            let mut value = match &mut self.tokens {
                Ok(Tokens::Comment(comment)) => match current {
                    '\r' | '\n' => Ok(Tokens::BreakLine),
                    v => {
                        comment.push(v);
                        continue;
                    }
                },
                Ok(Tokens::Sign(previous)) => match current {
                    '\u{0000}'..='\u{0008}' | '\u{000E}'..='\u{001F}' => {
                        Err(TokenError::ControlCode)
                    }
                    '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | ':'
                    | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`'
                    | '{' | '|' | '}' | '~' => {
                        todo!()
                    }
                    '/' => {
                        if match *previous {
                            Sign::Single(Single::Slash) => true,
                            _ => false,
                        } {
                            self.tokens = Ok(Tokens::Comment(String::from("")));
                            continue;
                        } else {
                            Ok(Tokens::Sign(Sign::Single(Single::Slash)))
                        }
                    }
                    '\'' | '"' => {
                        todo!()
                    }
                    ' ' | '\t' => {
                        todo!()
                    }
                    '\r' | '\n' => {
                        todo!()
                    }
                    '0'..='9' => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                },
                Ok(Tokens::Constant(s)) => todo!(),
                Ok(Tokens::Ident(s)) => todo!(),
                Ok(Tokens::KeyWords(s)) => todo!(),
                Ok(_) => todo!(),
                Err(_) => todo!(),
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

#[cfg(test)]
mod token_test {
    use super::{single_map, TokenStream};
    #[test]
    fn test_comment_token() {
        for i in TokenStream::from("// I am the one") {
            println!("{:?}", i);
        }
    }
    #[test]
    fn test_compile_time() {
        for (index, v) in single_map().iter().enumerate() {
            let c = index as u8 as char;
            if c.is_ascii_punctuation() {
                assert!(v.is_some());
                let x: char = v.unwrap().into();
                assert_eq!(c, x);
            }
        }
    }
    #[test]
    fn token_stream() {
        let content = std::fs::read_to_string("src.lq").unwrap();
        for token in TokenStream::from(&content) {
            println!("{:?} ", token);
        }
    }
}
