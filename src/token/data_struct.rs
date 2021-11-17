use std::{
    convert::{TryFrom, TryInto},
    ops::Add,
};

use colored::{ColoredString, Colorize};

use crate::{ConstArray, ConvertToSign, Double, KeyWords, SignAdd, Single, Triple};

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
    ColonGreater                  => ":>",
    GreaterColon                  => "<:",
    TildeGreater                  => "~>",
}

ConstArray!(
    NUMBER_TYPE,
    [i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,]
);

KeyWords! ({Debug, Clone, Copy},
    Back ,Move, Fun, Yun, Yield, Async, Await, Trait,
    Implement, For, Bind, Type, Enum, Struct,
    Parallel, Cast, Tobe, Module, Where, Loop,
    While, When, Match, Macro, Public, Dynamic,
    Box, Atomic, Const, Static, Lazy, In, From,
    To, Reference, Extern, Do, Algin, Mutable,
    Block, Expression, Let
);

#[derive(Debug)]
pub enum Constant {
    String(String),
    Char(char),
    Number(String),
}

impl Constant {
    pub fn len(&self) -> u32 {
        match self {
            Constant::Char(_) => 1,
            Constant::String(s) | Constant::Number(s) => s.len() as u32,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Constant::Char(_) => false,
            Constant::String(s) | Constant::Number(s) => s.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignError {
    NotSign,
    NotMatch,
}

Triple! {
    EqualEqualEqual => "===",
    PeriodPeriodPeriod => "...",
    ExclamationEqualEqual => "!==",
    GreaterGreaterGreater => ">>>",
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
        Slash,Slash
        Tilde,Greater
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
            (Self::Colon, Self::Greater) => Some(ColonGreater),
            (Self::Greater, Self::Colon) => Some(GreaterColon),
            (Self::Tilde, Self::Greater) => Some(TildeGreater),
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
    pub fn len(&self) -> u32 {
        match self {
            Self::Single(_) => 1,
            Self::Double(_) => 2,
            Self::Triple(_) => 3,
            Self::Quadruple(_) => 4,
        }
    }
    pub fn is_empty(&self) -> bool {
        false
    }
}

ConvertToSign!(Single);
ConvertToSign!(Double);
ConvertToSign!(Triple);
ConvertToSign!(Quadruple);

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

impl Tokens {
    pub fn to_string(&self) -> ColoredString {
        match self {
            Tokens::Ident(s) => s.to_string().white(),
            Tokens::KeyWords(s) => s.to_string().red(),
            Tokens::Constant(s) => match s {
                Constant::String(s) => s.to_string(),
                Constant::Char(c) => c.to_string(),
                Constant::Number(n) => n.to_string(),
            }
            .yellow()
            .bold(),
            Tokens::Comment(s) => s.to_string().green().italic(),
            Tokens::Sign(s) => match s {
                Sign::Single(s) => {
                    let x: char = (*s).into();
                    x
                }
                .to_string(),
                Sign::Double(v) => v.to_string(),
                Sign::Triple(v) => v.to_string(),
                Sign::Quadruple(_) => todo!(),
            }
            .blue()
            .underline(),
            Tokens::Empty => " ".to_string().black(),
            Tokens::BreakLine => "\n".to_string().black(),
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
    pub content: Tokens,
    pub line: u32,
    pub col: [u32; 2],
}

#[derive(Debug)]
pub enum TokenError {
    ControlCode,
    InvalidUnicode,
    InvalidIdent,
    InvalidCharDefine,
    InvalidNumberDefine,
    InvalidStringDefine,
}
#[derive(Debug)]
pub enum State<K, U> {
    Known(K),
    Unknown(U),
}

impl TryFrom<char> for Sign {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let single: Result<Single, SignError> = value.try_into();
        match single {
            Ok(v) => Ok(Sign::Single(v)),
            Err(_) => Err(value),
        }
    }
}

impl TryFrom<char> for Tokens {
    type Error = TokenError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\u{0000}'..='\u{0008}' | '\u{000E}'..='\u{001F}' => Err(TokenError::ControlCode),
            '0'..='9' => Ok(Tokens::Constant(Constant::Number(String::from(value)))),
            '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | ':' | ';'
            | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|'
            | '}' | '~' | '/' | '\'' | '"' => {
                Ok(Tokens::Sign(Single::try_from(value).unwrap().into()))
            }
            // 'F' => Ok(KeyWords::F.into()),
            // 'Y' => Ok(KeyWords::Y.into()),
            ' ' => Ok(Tokens::Empty),
            '\r' => Ok(Tokens::Empty),
            '\n' => Ok(Tokens::BreakLine),
            v => Ok(Tokens::Ident(v.to_string())),
        }
    }
}
#[inline]
fn is_sign(v: char) -> bool {
    matches!(
        v,
        '!' | '#'
            | '$'
            | '%'
            | '&'
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~'
            | '/'
            | '\''
            | '"'
    )
}

// #[cfg(test)]
// mod token_test {
//     use super::{super::TokenStream, single_map};
//     #[test]
//     fn test_comment_token() {
//         for i in TokenStream::from(&std::fs::read_to_string("src.lq").unwrap()) {
//             println!("{:?}", i.unwrap().content);
//         }
//     }
//     #[test]
//     fn test_compile_time() {
//         for (index, v) in single_map().iter().enumerate() {
//             let c = index as u8 as char;
//             if c.is_ascii_punctuation() {
//                 assert!(v.is_some());
//                 let x: char = v.unwrap().into();
//                 assert_eq!(c, x);
//             }
//         }
//     }
//     #[test]
//     fn token_stream() {
//         let content = std::fs::read_to_string("src.lq").unwrap();
//         for token in TokenStream::from(&content) {
//             println!("{:?} ", token);
//         }
//     }
// }
