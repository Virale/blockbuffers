use position::VectorPosition;
use std::ops::Deref;
use std::slice;

/// Vector wraps the buffer and the vector position.
///
/// # Examples
///
/// ```
/// use fbg::{Vector, position::VectorPosition, Scalar};
///
/// let buf = &[02u8, 0, 0, 0, 1, 0, 2, 0, 3, 0][..];
/// let pos = VectorPosition::<Scalar<u16>>::new(0);
/// let vector = Vector::new(buf, pos);
///
/// assert_eq!(2, vector.len());
/// assert_eq!(
///     &[
///         Scalar::from_little_endian(1u16),
///         Scalar::from_little_endian(2u16)
///     ],
///     vector.as_slice()
/// );
/// let collected: Vec<u16> = vector.iter().map(Into::into).collect();
/// assert_eq!(vec![1u16, 2], collected);
/// assert_eq!(1u16, vector[0].into());
/// assert_eq!(None, vector.get(2));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vector<T, I> {
    buf: T,
    pos: VectorPosition<I>,
}

impl<T, I> Vector<T, I> {
    pub fn new(buf: T, pos: VectorPosition<I>) -> Vector<T, I> {
        Vector { buf, pos }
    }

    pub fn buffer(&self) -> &T {
        &self.buf
    }

    pub fn position(&self) -> &VectorPosition<I> {
        &self.pos
    }
}

impl<'a, T: Clone, I> Vector<&'a T, I> {
    /// Clones the underlying buffer to create an owned string.
    pub fn into_owned(self) -> Vector<T, I> {
        Vector {
            buf: self.buf.clone(),
            pos: self.pos,
        }
    }
}

impl<T: AsRef<[u8]>, I> Vector<T, I> {
    pub fn len(&self) -> usize {
        self.pos.items_len(&self.buf)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the serialized vector in buffer as slice.
    pub fn as_slice(&self) -> &[I] {
        self.pos.as_slice(&self.buf)
    }

    pub fn iter(&self) -> slice::Iter<I> {
        self.as_slice().into_iter()
    }
}

impl<T: AsRef<[u8]>, I> AsRef<[I]> for Vector<T, I> {
    fn as_ref(&self) -> &[I] {
        self.as_slice()
    }
}

impl<T: AsRef<[u8]>, I> Deref for Vector<T, I> {
    type Target = [I];

    fn deref(&self) -> &[I] {
        self.as_slice()
    }
}
