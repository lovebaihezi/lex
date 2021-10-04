use std::{
    cell::RefCell,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    mem::swap,
    ops::Add,
    rc::Rc,
    thread::__FastLocalKeyInner,
};

use crate::word::{Word, WordError, WordStream, Words};
#[derive(Debug, Clone)]
pub enum Constant {
    Number(String),
    Chars(char),
    Strings(String),
}

#[derive(Debug, Clone)]
pub enum KeyWords {
    Back,
    Move,
    F,
    Y,
    Yield,
    Async,
    Await,
    Trait,
    Implement,
    For,
    Bind,
    Type,
    Enum,
    Struct,
    Parallel,
    Cast,
    Tobe,
    Module,
    Where,
    Loop,
    While,
    When,
    Match,
    Macro,
    Public,
    Dynamic,
    Box,
    Atomic,
    Const,
    Static,
    Lazy,
    In,
    From,
    To,
    Reference,
    Extern,
    Do,
    Algin,
    Mutable,
    Block,
    Expression,
}

impl ToString for KeyWords {
    fn to_string(&self) -> String {
        match self {
            Self::Back => "back",
            Self::Move => "move",
            Self::F => "F",
            Self::Y => "Y",
            Self::Yield => "yield",
            Self::Async => "async",
            Self::Await => "await",
            Self::Trait => "trait",
            Self::Implement => "implement",
            Self::For => "for",
            Self::Bind => "bind",
            Self::Type => "type",
            Self::Enum => "enum",
            Self::Struct => "struct",
            Self::Parallel => "parallel",
            Self::Cast => "cast",
            Self::Tobe => "tobe",
            Self::Module => "module",
            Self::Where => "where",
            Self::Loop => "loop",
            Self::While => "while",
            Self::When => "when",
            Self::Match => "match",
            Self::Macro => "macro",
            Self::Public => "public",
            Self::Dynamic => "dynamic",
            Self::Box => "box",
            Self::Atomic => "atomic",
            Self::Const => "const",
            Self::Static => "static",
            Self::Lazy => "lazy",
            Self::In => "in",
            Self::From => "from",
            Self::To => "to",
            Self::Reference => "reference",
            Self::Extern => "extern",
            Self::Do => "do",
            Self::Algin => "algin",
            Self::Mutable => "mutable",
            Self::Block => "block",
            Self::Expression => "expression",
        }
        .to_string()
    }
}

impl KeyWords {
    fn length(&self) -> u32 {
        match self {
            KeyWords::Back => 4,
            KeyWords::Move => 4,
            KeyWords::F => 1,
            KeyWords::Y => 1,
            KeyWords::Yield => 5,
            KeyWords::Async => 5,
            KeyWords::Await => 5,
            KeyWords::Trait => 5,
            KeyWords::Implement => 9,
            KeyWords::For => 3,
            KeyWords::Bind => 4,
            KeyWords::Type => 4,
            KeyWords::Enum => 4,
            KeyWords::Struct => 6,
            KeyWords::Parallel => 8,
            KeyWords::Cast => 4,
            KeyWords::Tobe => 4,
            KeyWords::Module => 6,
            KeyWords::Where => 5,
            KeyWords::Loop => 4,
            KeyWords::While => 5,
            KeyWords::When => 4,
            KeyWords::Match => 5,
            KeyWords::Macro => 5,
            KeyWords::Public => 6,
            KeyWords::Dynamic => 7,
            KeyWords::Box => 3,
            KeyWords::Atomic => 6,
            KeyWords::Const => 5,
            KeyWords::Static => 6,
            KeyWords::Lazy => 4,
            KeyWords::In => 2,
            KeyWords::From => 4,
            KeyWords::To => 2,
            KeyWords::Reference => 9,
            KeyWords::Extern => 6,
            KeyWords::Do => 2,
            KeyWords::Algin => 5,
            KeyWords::Mutable => 7,
            KeyWords::Block => 5,
            KeyWords::Expression => 10,
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

const fn create_ascii_map() -> [Result<Single, SignError>; 126] {
    let mut arr = [Err(SignError::NotSign); 126];
    let mut index = 0u8;
    while index < 126 {
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

static MAP: [Result<Single, SignError>; 126] = create_ascii_map();

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

#[derive(Debug)]
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
    DivideEqual,                   // /=
    HashHash,                      // ##
    AmpersandAmpersand,            // &&
    CaretCaret,                    // ^^
    LeftCurlyBracketVerticalBar,   // {|
    VerticalBarRightCurlyBracket,  // |}
    LeftSquareBracketVerticalBar,  // [|
    VerticalBarRightSquareBracket, // |]
}

#[derive(Debug)]
pub enum Triple {
    EqualEqualEqual,       // ===
    PeriodPeriodPeriod,    // ...
    ExclamationEqualEqual, // !==
    GreaterGreaterGreater, // >>>
}

#[derive(Debug)]
pub enum Quadruple {}

impl Add for Single {
    type Output = Option<Double>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add<Double> for Single {
    type Output = Option<Triple>;

    fn add(self, rhs: Double) -> Self::Output {
        match (self, rhs) {
            (Single::Equal, Double::EqualEqual) => Some(Triple::EqualEqualEqual),
            _ => None,
        }
    }
}

impl Add<Single> for Double {
    type Output = Option<Triple>;

    fn add(self, rhs: Single) -> Self::Output {
        todo!()
    }
}

#[derive(Debug)]
pub enum Sign {
    Single(Single),
    Double(Double),
    Triple(Triple),
    Quadruple(Quadruple),
}

impl Sign {
    fn length(&self) -> u32 {
        match self {
            Self::Single(_) => 1,
            Self::Double(_) => 2,
            Self::Triple(_) => 3,
            Self::Quadruple(_) => 4,
        }
    }
}

impl Add for Sign {
    type Output = Option<Sign>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[derive(Debug)]
pub enum Tokens {
    Identify(String),
    KeyWords(KeyWords),
    Constant(Constant),
    Comment(String),
    Sign(Sign),
    Empty,
    BreakLine,
}

impl Default for Tokens {
    #[inline]
    fn default() -> Self {
        Self::Empty
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
    col: [u32; 2],
}

#[derive(Debug)]
pub enum TokenError {
    ControlCode,
    InvalidUnicode,
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
        match &self.tokens {
            Ok(v) => match v {
                Tokens::Identify(s) => self.col[1] += s.len() as u32,
                Tokens::KeyWords(key_word) => self.col[1] += key_word.length(),
                Tokens::Constant(constant) => todo!(),
                Tokens::Comment(comment) => todo!(),
                Tokens::Sign(sign) => self.col[1] += sign.length(),
                Tokens::Empty => self.col[1] += 1,
                Tokens::BreakLine => self.line += 1,
            },
            _ => {}
        }
        swap(tokens, &mut self.tokens);
        (self.line, self.col)
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
        while let Some(word) = self.stream.next() {}
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
    use super::create_ascii_map;

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
}
