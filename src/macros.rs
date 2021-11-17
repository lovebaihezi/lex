#[macro_export]
macro_rules! KeyWords {
    ({$($derive: path),*$(,)*}, $($id:ident),*$(,)*) => {
        #[derive($($derive),*)]
        pub enum KeyWords {
            $($id,)+
        }
        const LEN:usize = [$(KeyWords::$id,)+].len();
        const KEYWORDS_ARRAY: [KeyWords; LEN] = [$(KeyWords::$id,)+];
        const KEYWORDS_STRING: [&'static str; LEN] = [$(stringify!($id),)+];
        const KEYWORDS_LENGTHS: [u32; LEN] = [$(stringify!($id).len() as u32,)+];
        impl KeyWords {
            #[inline]
            pub fn len(&self) -> u32 {
                unsafe { *KEYWORDS_LENGTHS.get_unchecked(*self as usize) }
            }
            #[inline]
            pub fn is_empty(&self) -> bool {
                false
            }
            #[inline]
            pub fn get_all<'a>() -> &'a [KeyWords] {
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
#[macro_export]
macro_rules! ConvertToSign {
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
#[macro_export]
macro_rules! SignAdd {
    {$(($v: path | $v1: path)) ,* $(,)*} => {
        impl Add for Sign {
            type Output = Option<Sign>;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(($v(v), $v1(v1)) => (v + v1).map(|v| v.into()),)+
                    _ => None,
                }
            }
        }
    };
}
#[macro_export]
macro_rules! ConstArray {
    ($name: ident, [$($id: ty),*$(,)*]) => {
        // const $name: [&'static str; [$(stringify!($id)),*].len()] = [$(stringify!($id),)*];
    };
}

#[macro_export]
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
        impl From<Single> for char {
            fn from(v: Single) -> Self {
                unsafe { *SINGLE_ARRAY.get_unchecked(v as usize) }
            }
        }
    };
}
#[macro_export]
macro_rules! Double {
    {$($left:ident => $real:expr),*$(,)*} => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Double {
            $($left,)*
        }
        // const DOUBLE_LEN: usize = [$($real),*].len();
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
#[macro_export]
macro_rules! Triple {
    {$($id:ident => $e:expr),*$(,)*} => {
        #[derive(Debug, Clone, Copy)]
        pub enum Triple {
            $($id,)*
        }
        impl ToString for Triple {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$id => $e,)*
                }.to_string()
            }
        }
    };
}
