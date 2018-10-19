use std::mem::size_of;

pub type UOffset = u32;
/// Signed offset used for vtable
pub type SOffset = i32;
/// Unsigned offset used for field offset stored in vtable.
pub type VOffset = u16;
/// Length of vector and string.
pub type Len = u32;

pub const SIZE_VOFFSET: usize = size_of::<VOffset>();
pub const SIZE_LEN: usize = size_of::<Len>();
