/// The trait `LE` converts between native endian and little endian.
pub trait LE {
    /// Converts a value in native endian to little endian.
    fn to_le(self) -> Self;
    /// Converts a value in little endian to native endian.
    fn from_le(x: Self) -> Self;
}

/// The macro `impl_le_for_enum` implements trait `LE` for enum.
///
/// The enum must specify a integer type via repr.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate fbg;
/// use fbg::le::LE;
///
/// #[repr(u16)]
/// enum Side {
///   Left = 1,
///   Right = 2,
/// }
/// impl_le_for_enum!(Side, u16);
///
/// assert_eq!(1u16, Side::from_le(Side::Left.to_le()) as u16);
/// assert_eq!(2u16, Side::from_le(Side::Right.to_le()) as u16);
/// ```
#[macro_export]
macro_rules! impl_le_for_enum {
    ($ty:ident, $repr:ident) => {{
        use fbg::le::LE;
        use std::mem::transmute;

        impl LE for $ty {
            fn to_le(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    unsafe { transmute((self as $repr).swap_bytes()) }
                }
            }
            fn from_le(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    unsafe { transmute((x as $repr).swap_bytes()) }
                }
            }
        }
    }};
}

macro_rules! impl_le_no_op {
    ($ty:ident) => {
        impl LE for $ty {
            fn to_le(self) -> Self {
                self
            }
            fn from_le(x: Self) -> Self {
                x
            }
        }
    };
}

impl_le_no_op!(bool);
impl_le_no_op!(i8);
impl_le_no_op!(u8);

macro_rules! impl_le_for_int {
    ($ty:ident) => {
        impl LE for $ty {
            fn to_le(self) -> Self {
                self.to_le()
            }
            fn from_le(x: Self) -> Self {
                Self::from_le(x)
            }
        }
    };
}

impl_le_for_int!(i16);
impl_le_for_int!(u16);
impl_le_for_int!(i32);
impl_le_for_int!(u32);
impl_le_for_int!(i64);
impl_le_for_int!(u64);

macro_rules! impl_le_for_float {
    ($ty:ident) => {
        impl LE for $ty {
            fn to_le(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    Self::from_bits(self.to_bits().swap_bytes())
                }
            }
            fn from_le(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    Self::from_bits(x.to_bits().swap_bytes())
                }
            }
        }
    };
}

impl_le_for_float!(f32);
impl_le_for_float!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        assert_eq!(true, bool::from_le(true.to_le()));
        assert_eq!(false, bool::from_le(false.to_le()));
        assert_eq!(1u8, u8::from_le(1u8.to_le()));
        assert_eq!(1u16, u16::from_le(1u16.to_le()));
        assert_eq!(1f32, f32::from_le(1f32.to_le()));

        assert_eq!(1u8, 1u8.to_le());

        #[cfg(target_endian = "little")]
        {
            assert_eq!(1u16, 1u16.to_le());
            assert_eq!(1u32, 1u32.to_le());
        }
        #[cfg(not(target_endian = "little"))]
        {
            assert_eq!(1u16.swap_bytes(), 1u16.to_le());
            assert_eq!(1u32.swap_bytes(), 1u32.to_le());
        }
    }
}
