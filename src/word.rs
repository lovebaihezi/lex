use std::{
    fmt::{Debug, Display},
    mem::swap,
    str::Chars,
};

#[derive(Debug, PartialEq)]
// comment will not process when in word stream
pub enum Words {
    Sign(char),
    Word(String),
    // Comment(String),
    Empty,
    BreakLine,
}

impl Default for Words {
    fn default() -> Self {
        Words::Empty
    }
}

impl Display for Words {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Words::Sign(v) => String::from(*v),
                Words::Word(s) => format!("{}", s),
                // Words::Comment(s) => format!("{}", s),
                Words::Empty => " ".to_string(),
                Words::BreakLine => String::from('\n'),
            }
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WordError {
    ControlCode,
}
// #[derive(Default)]
// pub struct Word {
//     pub content: Words,
// }

// impl Debug for Word {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!(
//             "{:?} line: {}, col: {}",
//             self.content,
//             self.line,
//             if self.col[0] == self.col[1] {
//                 format!("{}", self.col[0])
//             } else {
//                 format!("{}~{}", self.col[0], self.col[1])
//             }
//         ))
//     }
// }

#[derive(Debug)]
pub struct WordStream<'a> {
    stream: Chars<'a>,
    word: Result<Words, WordError>,
    // line: u32,
    // col: [u32; 2],
}

impl<'a, 'b: 'a> WordStream<'a> {
    #[inline]
    fn new(stream: &'b str) -> Self {
        Self {
            stream: stream.chars(),
            word: Ok(Default::default()),
            // line: 0,
            // col: Default::default(),
        }
    }
}

impl<'a, 'b: 'a> From<&'b String> for WordStream<'a> {
    #[inline]
    fn from(v: &'b String) -> Self {
        Self::new(v.as_str())
    }
}

impl<'a, 'b: 'a> From<&'b str> for WordStream<'a> {
    #[inline]
    fn from(v: &'b str) -> Self {
        Self::new(v)
    }
}

impl<'a> Iterator for WordStream<'a> {
    type Item = Result<Words, WordError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.stream.next() {
            let mut v = if c.is_whitespace() {
                if c == '\n' {
                    Ok(Words::BreakLine)
                } else {
                    Ok(Words::Empty)
                }
            } else if c.is_control() {
                Err(WordError::ControlCode)
            } else if c.is_ascii_punctuation() {
                Ok(Words::Sign(c))
            } else {
                match &mut self.word {
                    Ok(Words::Word(s)) => {
                        s.push(c);
                        continue;
                    }
                    _ => Ok(Words::Word(String::from(c))),
                }
            };
            swap(&mut v, &mut self.word);
            return Some(v.and_then(|content| Ok(content)));
        }
        match &self.word {
            Ok(Words::Empty) => None,
            _ => {
                let mut empty = Ok(Words::Empty);
                swap(&mut empty, &mut self.word);
                Some(empty.and_then(|content| Ok(content)))
            }
        }
    }
}

#[cfg(test)]
mod word_test {

    use super::WordStream;
    #[test]
    fn word_stream_example() {
        //            0123456789ABCD
        //                             0123456789ABC
        let s = "   [>123<}  123\n456(>7891123;    ";
        // let mut self = WordStream::from(s.clone());
        for i in WordStream::from(s.clone()) {
            println!("{:?}", i.unwrap());
        }
        // assert_eq!(self.next().unwrap().unwrap().word, Words::Sign('['));
        // assert_eq!(
        //     self.next().unwrap().unwrap().word,
        //     Words::Word("1".to_string())
        // );
        // assert_eq!(self.next().unwrap().unwrap().word, Words::Sign('}'));
        // assert_eq!(
        //     self.next().unwrap().unwrap().word,
        //     Words::Word("2".to_string())
        // );
        // assert_eq!(self.next().unwrap().unwrap().word, Words::Sign('('));
        // assert_eq!(
        //     self.next().unwrap().unwrap().word,
        //     Words::Word("3".to_string())
        // );
        // assert_eq!(self.next().unwrap().unwrap().word, Words::Sign(';'));
    }

    #[test]
    fn word_stream_test_read_code() {
        let s = std::fs::read_to_string("src.lq").unwrap();
        for i in WordStream::from(&s.clone()) {
            println!("{:?}", i.unwrap());
        }
    }
    #[test]
    fn sample() {
        let i = '\n';
        println!("{} {}", i.is_whitespace(), i.is_control());
    }
}
