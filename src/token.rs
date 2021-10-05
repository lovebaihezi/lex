use crate::word::{WordError, WordStream, Words};
use std::{
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

KeyWords! ({Debug, Clone, Copy},
    Back ,Move, F, Y, Yield, Async, Await, Trait,
    Implement, For, Bind, Type, Enum, Struct,
    Parallel, Cast, Tobe, Module, Where, Loop,
    While, When, Match, Macro, Public, Dynamic,
    Box, Atomic, Const, Static, Lazy, In, From,
    To, Reference, Extern, Do, Algin, Mutable,
    Block, Expression,
);

macro_rules! ConstArray {
    ($name: ident, [$($id: ty),*$(,)*]) => {
        const $name: [&'static str; [$(stringify!($id)),*].len()] = [$(stringify!($id),)*];
    };
}

ConstArray!(
    NUMBER_TYPE,
    [i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,]
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
            //  => s.len() as u32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignError {
    NotSign,
    NotMatch,
}

#[derive(Debug, Clone, Copy)]
pub enum Single {
    Exclamation,        // !
    Quotation,          // "
    Hash,               // #
    Dollar,             // $
    Percent,            // %
    Ampersand,          // &
    Apostrophe,         // '
    LeftParenthesis,    // (
    RightParenthesis,   // )
    Asterisk,           // *
    Plus,               // +
    Comma,              // ,
    Minus,              // -
    Period,             // .
    Slash,              // /
    Colon,              // :
    Semicolon,          // ;
    Less,               // <
    Equal,              // =
    Greater,            // >
    Question,           // ?
    At,                 // @
    LeftSquareBracket,  // [
    BackSlash,          // \
    RightSquareBracket, // ]
    Caret,              // ^
    UnderScore,         // _
    GraveAccent,        // `
    LeftCurlyBracket,   // {
    VerticalBar,        // |
    RightCurlyBracket,  // }
    Tilde,              // ~
}

const fn create_ascii_map() -> [Result<Single, SignError>; 127] {
    let mut arr = [Err(SignError::NotSign); 127];
    let mut index = 0u8;
    while index <= 126 {
        arr[index as usize] = match index as char {
            '!' => Ok(Single::Exclamation),
            '"' => Ok(Single::Quotation),
            '#' => Ok(Single::Hash),
            '$' => Ok(Single::Dollar),
            '%' => Ok(Single::Percent),
            '&' => Ok(Single::Ampersand),
            '\'' => Ok(Single::Apostrophe),
            '(' => Ok(Single::LeftParenthesis),
            ')' => Ok(Single::RightParenthesis),
            '*' => Ok(Single::Asterisk),
            '+' => Ok(Single::Plus),
            ',' => Ok(Single::Comma),
            '-' => Ok(Single::Minus),
            '.' => Ok(Single::Period),
            '/' => Ok(Single::Slash),
            ':' => Ok(Single::Colon),
            ';' => Ok(Single::Semicolon),
            '<' => Ok(Single::Less),
            '=' => Ok(Single::Equal),
            '>' => Ok(Single::Greater),
            '?' => Ok(Single::Question),
            '@' => Ok(Single::At),
            '[' => Ok(Single::LeftSquareBracket),
            '\\' => Ok(Single::BackSlash),
            ']' => Ok(Single::RightSquareBracket),
            '^' => Ok(Single::Caret),
            '_' => Ok(Single::UnderScore),
            '`' => Ok(Single::GraveAccent),
            '{' => Ok(Single::LeftCurlyBracket),
            '|' => Ok(Single::VerticalBar),
            '}' => Ok(Single::RightCurlyBracket),
            '~' => Ok(Single::Tilde),
            _ => Err(SignError::NotSign),
        };
        index += 1;
    }
    arr
}

static MAP: [Result<Single, SignError>; 127] = create_ascii_map();

impl TryFrom<char> for Single {
    type Error = SignError;
    fn try_from(v: char) -> Result<Self, Self::Error> {
        if v > '\u{007E}' {
            Err(SignError::NotSign)
        } else {
            MAP[v as u8 as usize]
        }
    }
}

impl Into<char> for Single {
    fn into(self) -> char {
        use Single::*;
        match self {
            Exclamation => '!',
            Quotation => '"',
            Hash => '#',
            Dollar => '$',
            Percent => '%',
            Ampersand => '&',
            Apostrophe => '\'',
            LeftParenthesis => '(',
            RightParenthesis => ')',
            Asterisk => '*',
            Plus => '+',
            Comma => ',',
            Minus => '-',
            Period => '.',
            Slash => '/',
            Colon => ':',
            Semicolon => ';',
            Less => '<',
            Equal => '=',
            Greater => '>',
            Question => '?',
            At => '@',
            LeftSquareBracket => '[',
            BackSlash => '\\',
            RightSquareBracket => ']',
            Caret => '^',
            UnderScore => '_',
            GraveAccent => '`',
            LeftCurlyBracket => '{',
            VerticalBar => '|',
            RightCurlyBracket => '}',
            Tilde => '~',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Double {
    EqualEqual,                    // ==
    EqualGreater,                  // =>
    LessEqual,                     // <=
    GreaterEqual,                  // >=
    EqualLess,                     // =<
    LessMinus,                     // <-
    MinusGreater,                  // ->
    VerticalBarGreater,            // |>
    ColonColon,                    // ::
    QuestionQuestion,              // ??
    ExclamationExclamation,        // !!
    VerticalBarVerticalBar,        // ||
    ColonEqual,                    // :=
    VerticalBarEqual,              // |=
    GreaterGreater,                // >>
    LessLess,                      // <<
    ExclamationEqual,              // !=
    LessVerticalBar,               // <|
    PeriodQuestion,                // .?
    PeriodExclamation,             // .!
    AsteriskEqual,                 // *=
    PlusEqual,                     // +=
    MinusEqual,                    // -=
    SlashEqual,                    // /=
    HashHash,                      // ##
    AmpersandAmpersand,            // &&
    CaretCaret,                    // ^^
    LeftCurlyBracketVerticalBar,   // {|
    VerticalBarRightCurlyBracket,  // |}
    LeftSquareBracketVerticalBar,  // [|
    VerticalBarRightSquareBracket, // |]
    PeriodPeriod,                  // ..
    SlashSlash,                    // //
}

impl Into<[char; 2]> for Double {
    fn into(self) -> [char; 2] {
        match self {
            Self::EqualEqual => ['=', '='],
            Self::EqualGreater => ['=', '>'],
            Self::LessEqual => ['<', '='],
            Self::GreaterEqual => ['>', '='],
            Self::EqualLess => ['=', '<'],
            Self::LessMinus => ['<', '-'],
            Self::MinusGreater => ['-', '>'],
            Self::VerticalBarGreater => ['|', '>'],
            Self::ColonColon => [':', ':'],
            Self::QuestionQuestion => ['?', '?'],
            Self::ExclamationExclamation => ['!', '!'],
            Self::VerticalBarVerticalBar => ['|', '|'],
            Self::ColonEqual => [':', '='],
            Self::VerticalBarEqual => ['|', '='],
            Self::GreaterGreater => ['>', '>'],
            Self::LessLess => ['<', '<'],
            Self::ExclamationEqual => ['!', '='],
            Self::LessVerticalBar => ['<', '|'],
            Self::PeriodQuestion => ['.', '?'],
            Self::PeriodExclamation => ['.', '!'],
            Self::AsteriskEqual => ['*', '='],
            Self::PlusEqual => ['+', '='],
            Self::MinusEqual => ['-', '='],
            Self::SlashEqual => ['/', '='],
            Self::HashHash => ['#', '#'],
            Self::AmpersandAmpersand => ['&', '&'],
            Self::CaretCaret => ['^', '^'],
            Self::LeftCurlyBracketVerticalBar => ['{', '|'],
            Self::VerticalBarRightCurlyBracket => ['|', '}'],
            Self::LeftSquareBracketVerticalBar => ['[', '|'],
            Self::VerticalBarRightSquareBracket => ['|', ']'],
            Self::PeriodPeriod => ['.', '.'],
            Self::SlashSlash => ['/', '/'],
        }
    }
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

#[derive(Debug)]
pub struct TokenStream<'a> {
    stream: WordStream<'a>,
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

impl From<WordError> for TokenError {
    #[inline]
    fn from(v: WordError) -> Self {
        match v {
            WordError::ControlCode => Self::ControlCode,
        }
    }
}

impl<'a, 'b: 'a> TokenStream<'a> {
    #[inline]
    fn new(v: &'b str) -> Self {
        TokenStream {
            stream: WordStream::from(v),
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

impl<'a, 'b: 'a> From<&'b str> for TokenStream<'a> {
    #[inline]
    fn from(v: &'b str) -> Self {
        Self::new(v)
    }
}

impl<'a, 'b: 'a> From<&'b String> for TokenStream<'a> {
    #[inline]
    fn from(v: &'b String) -> Self {
        Self::new(v.as_str())
    }
}

impl<'a> Iterator for TokenStream<'a> {
    fn next(self: &mut TokenStream<'a>) -> Option<Self::Item> {
        while let Some(word) = self.stream.next() {
            let mut value: Result<Tokens, TokenError> = match word {
                Ok(v) => match v {
                    Words::Sign(sign) => match {
                        let s: Result<Single, SignError> = sign.try_into();
                        s
                    } {
                        Ok(s) => {
                            let s: Sign = s.into();
                            Ok(Tokens::Sign(match self.tokens {
                                Ok(Tokens::Sign(x)) => match x + s {
                                    Some(_) => continue,
                                    None => s,
                                },
                                _ => s,
                            }))
                        }
                        _ => Err(TokenError::InvalidUnicode),
                    },
                    Words::Word(word) => {
                        let key: Result<KeyWords, &str> = word.as_str().try_into();
                        match key {
                            Ok(key_words) => Ok(Tokens::KeyWords(key_words)),
                            Err(_) => {
                                todo!()
                            }
                        }
                    }
                    Words::Empty => Ok(Tokens::Empty),
                    Words::BreakLine => Ok(Tokens::BreakLine),
                },
                Err(v) => Err(v.into()),
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
    use super::{create_ascii_map, TokenStream};

    #[test]
    fn test_compile_time() {
        for (index, v) in create_ascii_map().iter().enumerate() {
            let c = index as u8 as char;
            if c.is_ascii_punctuation() {
                assert!(v.is_ok());
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
