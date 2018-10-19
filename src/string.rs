use position::StringPosition;
use std::ops::Deref;

/// String wraps the buffer and the string position.
///
/// # Example
///
/// ```
/// use fbg::{String, position::StringPosition};
///
/// let buf = &[03u8, 0, 0, 0, 'f' as u8, 'b' as u8, 'g' as u8, 0][..];
/// let string = String::new(buf, StringPosition::new(0));
///
/// assert_eq!("fbg", string.as_str());
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct String<T> {
    buf: T,
    pos: StringPosition,
}

impl<T> String<T> {
    pub fn new(buf: T, pos: StringPosition) -> String<T> {
        String { buf, pos }
    }

    pub fn buffer(&self) -> &T {
        &self.buf
    }

    pub fn position(&self) -> &StringPosition {
        &self.pos
    }
}

impl<'a, T: Clone> String<&'a T> {
    /// Clones the underlying buffer to create an owned string.
    pub fn into_owned(self) -> String<T> {
        String {
            buf: self.buf.clone(),
            pos: self.pos,
        }
    }
}

impl<T: AsRef<[u8]>> String<T> {
    /// Returns the serialized string in buffer.
    pub fn as_str(&self) -> &str {
        self.pos.as_str(&self.buf)
    }
}

impl<T: AsRef<[u8]>> AsRef<str> for String<T> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<T: AsRef<[u8]>> Deref for String<T> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}
