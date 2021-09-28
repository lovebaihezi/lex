use std::{cell::RefCell, rc::Rc, str::Chars};

#[derive(Debug, PartialEq)]
// comment will not process when in word stream
pub enum Words {
    Sign(char),
    Word(String),
}

#[derive(Debug)]
pub enum WordError {
    ControlCode,
}

#[derive(Debug)]
pub struct WordStream<'a> {
    stream: Chars<'a>,
    word: String,
}

impl<'a, 'b: 'a> WordStream<'a> {
    #[inline]
    fn new(stream: &'b str) -> Self {
        Self {
            stream: stream.chars(),
            word: Default::default(),
        }
    }
}

impl<'a, 'b: 'a> From<&'b String> for WordStream<'a> {
    fn from(v: &'b String) -> Self {
        Self {
            stream: v.chars(),
            word: Default::default(),
        }
    }
}

impl<'a, 'b: 'a> From<&'b str> for WordStream<'a> {
    fn from(v: &'b str) -> Self {
        Self {
            stream: v.chars(),
            word: Default::default(),
        }
    }
}

impl<'a> Iterator for WordStream<'a> {
    type Item = Result<Words, WordError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::new();
        while let Some(c) = self.stream.next() {
            if c.is_whitespace() {
                if s.is_empty() {
                    continue;
                } else {
                    return Some(Ok(Words::Word(s)));
                }
            } else if c.is_control() {
                return Some(Err(WordError::ControlCode));
            } else if c.is_ascii_punctuation() {
                return Some(Ok(Words::Sign(c)));
            } else {
                s.push(c);
            }
        }
        if s.is_empty() {
            None
        } else {
            Some(Ok(Words::Word(s)))
        }
    }
}

#[cfg(test)]
mod word_test {
    use crate::word::Words;

    use super::WordStream;

    #[test]
    fn word_stream_example() {
        let mut ws = WordStream::from(r###"[123] [123]"###);
        for i in WordStream::from(r###"[123] [123]"###) {
            println!("{:?}", i.unwrap());
        }
        assert_eq!(ws.next().unwrap().unwrap(), Words::Sign('['));
        assert_eq!(ws.next().unwrap().unwrap(), Words::Word("123".to_string()));
        assert_eq!(ws.next().unwrap().unwrap(), Words::Sign(']'));
        assert_eq!(ws.next().unwrap().unwrap(), Words::Sign('['));
        assert_eq!(ws.next().unwrap().unwrap(), Words::Word("123".to_string()));
        assert_eq!(ws.next().unwrap().unwrap(), Words::Sign(']'));
    }
}
