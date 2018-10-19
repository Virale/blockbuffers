use byteorder::{ByteOrder, LittleEndian};

pub trait Read {
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self;
}

impl Read for bool {
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        buf.as_ref()[pos] != 0
    }
}

impl Read for u8 {
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        buf.as_ref()[pos]
    }
}

impl Read for i8 {
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        buf.as_ref()[pos] as i8
    }
}

macro_rules! impl_read_via_byteorder {
    ($ty:ident, $func:ident) => {
        impl Read for $ty {
            fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
                LittleEndian::$func(&buf.as_ref()[pos..])
            }
        }
    };
}

impl_read_via_byteorder!(u16, read_u16);
impl_read_via_byteorder!(i16, read_i16);
impl_read_via_byteorder!(u32, read_u32);
impl_read_via_byteorder!(i32, read_i32);
impl_read_via_byteorder!(u64, read_u64);
impl_read_via_byteorder!(i64, read_i64);
impl_read_via_byteorder!(f32, read_f32);
impl_read_via_byteorder!(f64, read_f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        assert_eq!(4, <u16>::read(&[4u8, 0, 0, 0, 1], 0));
    }
}
