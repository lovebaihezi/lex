use std::{cell::RefCell, convert::TryFrom, rc::Rc};

use crate::word::{Word, WordStream};
#[derive(Debug, Clone)]
pub enum Constant {
    Number(RefCell<String>),
    Chars(RefCell<char>),
    Strings(RefCell<String>),
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
}

impl ToString for KeyWords {
    fn to_string(&self) -> String {
        match self {
            KeyWords::Back => "back",
            KeyWords::Move => "move",
            KeyWords::F => "f",
            KeyWords::Y => "y",
            KeyWords::Yield => "yield",
            KeyWords::Async => "async",
            KeyWords::Await => "await",
            KeyWords::Trait => "trait",
            KeyWords::Implement => "implement",
            KeyWords::For => "for",
            KeyWords::Bind => "bind",
            KeyWords::Type => "type",
            KeyWords::Enum => "enum",
            KeyWords::Struct => "struct",
            KeyWords::Parallel => "parallel",
            KeyWords::Cast => "cast",
            KeyWords::Tobe => "tobe",
            KeyWords::Module => "module",
            KeyWords::Where => "where",
            KeyWords::Loop => "loop",
            KeyWords::While => "while",
            KeyWords::When => "when",
            KeyWords::Match => "match",
            KeyWords::Macro => "macro",
            KeyWords::Public => "public",
            KeyWords::Dynamic => "dynamic",
            KeyWords::Box => "box",
            KeyWords::Atomic => "atomic",
            KeyWords::Const => "const",
            KeyWords::Static => "static",
            KeyWords::Lazy => "lazy",
            KeyWords::In => "in",
            KeyWords::From => "from",
            KeyWords::To => "to",
            KeyWords::Reference => "reference",
            KeyWords::Extern => "extern",
            KeyWords::Do => "do",
            KeyWords::Algin => "algin",
            KeyWords::Mutable => "mutable",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Sign {
    Add,
    Minus,
    Multiply,
    Divide,
}

impl TryFrom<String> for KeyWords {
    fn try_from(v: String) -> Result<Self, Self::Error> {
        todo!()
    }
    type Error = TokenError;
}

#[derive(Debug, Clone)]
pub enum Tokens {
    Identify(RefCell<String>),
    KeyWords(RefCell<KeyWords>),
    Constant(RefCell<Constant>),
    Macros(RefCell<String>),
    Comment(RefCell<String>),
    Empty,
}

#[derive(Debug)]
pub struct TokenStream<'a> {
    stream: WordStream<'a>,
}

impl From<Word> for Tokens {
    fn from(_: Word) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum TokenError {
    ControlCode,
    InvalidUnicode,
}

impl<'a, 'b: 'a> From<&'b str> for TokenStream<'a> {
    #[inline]
    fn from(v: &'b str) -> Self {
        TokenStream {
            stream: WordStream::from(v),
        }
    }
}

impl<'a, 'b: 'a> From<&'b String> for TokenStream<'a> {
    #[inline]
    fn from(v: &'b String) -> Self {
        TokenStream {
            stream: WordStream::from(v),
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    fn next(self: &mut TokenStream<'a>) -> Option<Self::Item> {
        while let Some(word) = self.stream.next() {}
        None
    }

    type Item = Result<Tokens, TokenError>;
}
