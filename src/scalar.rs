use std::mem::size_of;

/// Scalar is a wrapper of scalars in little endian.
///
/// It is a convention to convert between little endian and native endian via `From` trait.
///
/// - `From<T> for Scalar<T>` converts scalar in native endian to little endian wrapped in `Scalar`.
/// - `From<Scalar<T>> for T` converts little endian wrapped in `Scalar` to native endian.
///
/// # Examples
///
/// ```
/// use fbg::Scalar;
///
/// assert_eq!(1u16, Scalar::from(1u16).into());
/// ```
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Scalar<T>(T);

impl<T> Scalar<T> {
    pub fn from_little_endian(value: T) -> Scalar<T> {
        Scalar(value)
    }

    pub fn into_little_endian(self) -> T {
        self.0
    }

    pub fn little_endian_ref(&self) -> &T {
        &self.0
    }

    pub fn little_endian_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Read the wrapper directly from buf.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::Scalar;
    ///
    /// let buf = &[1u8, 0];
    /// let scalar = Scalar::<u16>::read_from(&[1u8, 0], 0);
    /// assert_eq!(1u16, scalar.into());
    /// ```
    pub fn read_from<B: AsRef<[u8]>>(buf: &B, pos: usize) -> &Scalar<T> {
        let size = size_of::<T>();
        let buf = &buf.as_ref()[pos..pos + size];
        let ptr = buf.as_ptr() as *const Scalar<T>;
        unsafe { &*ptr }
    }
}

/// Implement `From` traits for flatbuffers enum.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate fbg;
/// use fbg::Scalar;
/// #[repr(u16)]
/// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
/// enum Side {
///     Left,
///     Right,
/// }
/// impl_scalar_convert_for_enum!(Side, u16);
///
/// assert_eq!(Side::Left, Scalar::from(Side::Left).into());
/// assert_eq!(Side::Right, Scalar::from(Side::Right).into());
/// ```
#[macro_export]
macro_rules! impl_scalar_convert_for_enum {
    ($ty:ident, $repr:ident) => {{
        use fbg::Scalar;
        use std::mem::transmute;
        impl From<Scalar<$ty>> for $ty {
            // Convert from little endian to native endian.
            fn from(value: Scalar<$ty>) -> Self {
                let n = <$repr>::from_le(value.into_little_endian() as $repr);
                unsafe { std::mem::transmute(n) }
            }
        }

        impl<'a> From<&'a Scalar<$ty>> for $ty {
            // Convert from little endian to native endian.
            fn from(value: &'a Scalar<$ty>) -> Self {
                let n = <$repr>::from_le(*value.little_endian_ref() as $repr);
                unsafe { std::mem::transmute(n) }
            }
        }

        impl From<$ty> for Scalar<$ty> {
            // Convert from native endian to little endian.
            fn from(value: $ty) -> Self {
                let n = (value as $repr).to_le();
                unsafe { std::mem::transmute(n) }
            }
        }
    }};
}

macro_rules! impl_for_noop {
    ($ty:ident) => {
        impl From<Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: Scalar<$ty>) -> $ty {
                value.0
            }
        }

        impl<'a> From<&'a Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: &'a Scalar<$ty>) -> $ty {
                value.0
            }
        }

        impl From<$ty> for Scalar<$ty> {
            // Converts from native endian to little endian stored in `Scalar`.
            #[inline(always)]
            fn from(value: $ty) -> Scalar<$ty> {
                Scalar(value)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($ty:ident) => {
        impl From<Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: Scalar<$ty>) -> $ty {
                <$ty>::from_le(value.0)
            }
        }

        impl<'a> From<&'a Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: &'a Scalar<$ty>) -> $ty {
                <$ty>::from_le(value.0)
            }
        }

        impl From<$ty> for Scalar<$ty> {
            // Converts from native endian to little endian stored in `Scalar`.
            #[inline(always)]
            fn from(value: $ty) -> Scalar<$ty> {
                Scalar(value.to_le())
            }
        }
    };
}

macro_rules! impl_for_float {
    ($ty:ident) => {
        impl From<Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: Scalar<$ty>) -> $ty {
                #[cfg(target_endian = "little")]
                {
                    value.0
                }
                #[cfg(not(target_endian = "little"))]
                {
                    <$ty>::from_bits(value.0.to_bits().swap_bytes())
                }
            }
        }

        impl<'a> From<&'a Scalar<$ty>> for $ty {
            // Converts from little endian stored in `Scalar` to native endian.
            #[inline(always)]
            fn from(value: &'a Scalar<$ty>) -> $ty {
                #[cfg(target_endian = "little")]
                {
                    value.0
                }
                #[cfg(not(target_endian = "little"))]
                {
                    <$ty>::from_bits(value.0.to_bits().swap_bytes())
                }
            }
        }

        impl From<$ty> for Scalar<$ty> {
            // Converts from native endian to little endian stored in `Scalar`.
            #[inline(always)]
            fn from(value: $ty) -> Scalar<$ty> {
                #[cfg(target_endian = "little")]
                {
                    Scalar(value)
                }
                #[cfg(not(target_endian = "little"))]
                {
                    <$ty>::from_bits(value.to_bits().swap_bytes())
                }
            }
        }
    };
}

impl_for_noop!(bool);
impl_for_noop!(u8);
impl_for_noop!(i8);
impl_for_int!(u16);
impl_for_int!(i16);
impl_for_int!(u32);
impl_for_int!(i32);
impl_for_int!(u64);
impl_for_int!(i64);
impl_for_float!(f32);
impl_for_float!(f64);

#[cfg(test)]
mod tests {
    use super::Scalar;

    #[test]
    fn test_convert() {
        assert_eq!(true, Scalar::from(true).into());
        assert_eq!(false, Scalar::from(false).into());
        assert_eq!(1u32, Scalar::from(1u32).into());
        assert_eq!(1f32, Scalar::from(1f32).into());
    }
}
