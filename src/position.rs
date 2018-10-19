use io::Read;
use std::marker::PhantomData;
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;
use types::SOffset;
use types::{Len, UOffset, VOffset, SIZE_LEN, SIZE_VOFFSET};

/// Calculates a new position using a signed backward offset.
///
/// If offset is positive, it is subtracted from pos.
/// Otherwise its absolute is added to pos.
///
/// # Examples
///
/// ```
/// use fbg::position::seek_soffset;
///
/// assert_eq!(3, seek_soffset(5, 2));
/// assert_eq!(7, seek_soffset(5, -2));
/// ```
pub fn seek_soffset(pos: usize, offset: SOffset) -> usize {
    pos.wrapping_sub(offset as usize)
}

/// Adds functions to check remaining bytes starting at the specified position.
pub trait BytesRemainingExt: AsRef<[u8]> {
    /// Tells whether there are remaining bytes in the buffer starting from the specified position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::BytesRemainingExt;
    /// let buf = &[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// assert_eq!(0, buf.remaining(11));
    /// assert_eq!(0, buf.remaining(10));
    /// assert_eq!(1, buf.remaining(9));
    /// assert_eq!(9, buf.remaining(1));
    /// assert_eq!(10, buf.remaining(0));
    /// ```
    fn has_remaining(&self, pos: usize) -> bool {
        pos < self.as_ref().len()
    }

    /// Tells the number of remaining bytes in the buffer starting from the specified position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::BytesRemainingExt;
    /// let buf = &[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    /// assert!(!buf.has_remaining(11));
    /// assert!(!buf.has_remaining(10));
    /// assert!(buf.has_remaining(9));
    /// assert!(buf.has_remaining(1));
    /// assert!(buf.has_remaining(0));
    /// ```
    fn remaining(&self, pos: usize) -> usize {
        self.as_ref().len().checked_sub(pos).unwrap_or_default()
    }
}

impl<T: AsRef<[u8]>> BytesRemainingExt for T {}

/// VectorPosition wrappers a position which points to a vector in the buffer.
///
/// Vectors are stored as contiguous aligned scalar elements prefixed by a 32bit element count.
///
/// # Examples
///
/// ```
/// use fbg::position::VectorPosition;
///
/// let buf = &[02u8, 0, 0, 0, 1, 0, 2, 0, 3, 0][..];
/// let pos = VectorPosition::<u16>::new(0);
///
/// assert_eq!(2, pos.items_len(&buf));
/// assert_eq!(&[1u16.to_le(), 2u16.to_le()], pos.as_slice(&buf));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct VectorPosition<I>(usize, PhantomData<I>);

impl<I> VectorPosition<I> {
    pub fn new(pos: usize) -> VectorPosition<I> {
        VectorPosition(pos, PhantomData)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn position(&self) -> usize {
        self.0
    }

    /// Reads the length of the vector.
    pub fn items_len<T: AsRef<[u8]>>(&self, buf: &T) -> usize {
        <Len>::read(&buf, self.0) as usize
    }

    pub fn items_len_raw_slice<'a, T: AsRef<[u8]>>(&self, buf: &'a T) -> &'a [u8] {
        &buf.as_ref()[..SIZE_LEN]
    }
}

impl<I: Sized> VectorPosition<I> {
    /// Gets the reference to the items slice.
    ///
    /// The slice attaches to the buffer directly, so all scalars are in little endian form.
    pub fn as_slice<T: AsRef<[u8]>>(&self, buf: &T) -> &[I] {
        let len = self.items_len(&buf);
        let start_pos = self.0 + SIZE_LEN;
        let end_pos = start_pos + len * size_of::<I>();
        let items_buf = &buf.as_ref()[start_pos..end_pos];

        let ptr = items_buf.as_ptr() as *const I;
        unsafe { from_raw_parts(ptr, len) }
    }
}

impl<I> Read for VectorPosition<I> {
    /// VectorPosition is stored as a UOffset in the buffer.
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        VectorPosition(pos + <UOffset>::read(&buf, pos) as usize, PhantomData)
    }
}

impl<I> AsRef<usize> for VectorPosition<I> {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

/// StringPosition wrappers a position which points to a string in the buffer.
///
/// Strings are stored as vectors of u8, and it is guaranteed that there will be an extra 0 after
/// the last item. The extra 0 does not count in length.
///
/// # Example
///
/// ```
/// use fbg::position::StringPosition;
///
/// let buf = &[03u8, 0, 0, 0, 'f' as u8, 'b' as u8, 'g' as u8, 0][..];
/// let pos = StringPosition::new(0);
///
/// assert_eq!(3, pos.bytes_len(&buf));
/// assert_eq!("fbg", pos.as_str(&buf));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct StringPosition(usize);

impl StringPosition {
    pub fn new(pos: usize) -> StringPosition {
        StringPosition(pos)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn position(self) -> usize {
        self.0
    }

    /// Reads the length of the string in bytes.
    pub fn bytes_len<T: AsRef<[u8]>>(self, buf: &T) -> usize {
        <Len>::read(&buf, self.0) as usize
    }

    pub fn bytes_len_raw_slice<'a, T: AsRef<[u8]>>(&self, buf: &'a T) -> &'a [u8] {
        &buf.as_ref()[..SIZE_LEN]
    }

    /// Gets the reference to the string.
    pub fn as_str<'a, T: AsRef<[u8]>>(&self, buf: &'a T) -> &'a str {
        let len = self.bytes_len(&buf);
        let start_pos = self.0 + SIZE_LEN;
        let end_pos = start_pos + len;
        unsafe { from_utf8_unchecked(&buf.as_ref()[start_pos..end_pos]) }
    }
}

impl Read for StringPosition {
    /// StringPosition is stored as a UOffset in the buffer.
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        StringPosition(pos + <UOffset>::read(&buf, pos) as usize)
    }
}

impl AsRef<usize> for StringPosition {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

/// VTablePosition wrappers a position which points to a vtable in the buffer.
///
/// The elements of a vtable are all of type voffset_t, which is a uint16_t. The first element is
/// the size of the vtable in bytes, including the size element. The second one is the size of the
/// object, in bytes (including the vtable offset). This size could be used for streaming, to
/// know how many bytes to read to be able to access all inline fields of the object. The
/// remaining elements are the N offsets, where N is the amount of fields declared in the schema
/// when the code that constructed this buffer was compiled (thus, the size of the table is N + 2).
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct VTablePosition(usize);

/// VTable has a head containing two VOffsets.
pub const SIZE_VTABLE_HEAD: usize = SIZE_VOFFSET + SIZE_VOFFSET;

impl VTablePosition {
    pub fn new(pos: usize) -> VTablePosition {
        VTablePosition(pos)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn position(self) -> usize {
        self.0
    }

    /// Reads the size of the vtable in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use fbg::position::VTablePosition;
    ///
    /// let buf = &[4u8, 0, 6, 0][..];
    /// let pos = VTablePosition::new(0);
    ///
    /// assert_eq!(4, pos.vtable_bytes_len(&buf));
    /// ```
    pub fn vtable_bytes_len<T: AsRef<[u8]>>(self, buf: &T) -> usize {
        <VOffset>::read(&buf, self.0) as usize
    }

    /// Reads the size of the table in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::VTablePosition;
    ///
    /// let buf = &[4u8, 0, 6, 0][..];
    /// let pos = VTablePosition::new(0);
    ///
    /// assert_eq!(6, pos.table_bytes_len(&buf));
    /// ```
    pub fn table_bytes_len<T: AsRef<[u8]>>(self, buf: &T) -> usize {
        <VOffset>::read(&buf, self.0 + SIZE_VOFFSET) as usize
    }

    /// Reads the field offset.
    ///
    /// Parameter `pos_in_vtable` is the position of the voffset inside vtable bytes. For example
    /// 4 means the offset for the first field in the schema.
    ///
    /// Offset 0 indicates the field is absent in the table bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::VTablePosition;
    ///
    /// // Field offsets are 20, 0, 4
    /// let buf = &[10u8, 0, 40, 0, 20, 0, 0, 0, 4, 0][..];
    /// let pos = VTablePosition::new(0);
    ///
    /// assert_eq!(20, pos.field_offset(&buf, 4));
    /// assert_eq!(0, pos.field_offset(&buf, 6));
    /// assert_eq!(4, pos.field_offset(&buf, 8));
    /// // Returns 0 when pos_in_vtable is out of range.
    /// assert_eq!(0, pos.field_offset(&buf, 10));
    /// ```
    pub fn field_offset<T: AsRef<[u8]>>(self, buf: &T, pos_in_vtable: usize) -> VOffset {
        if pos_in_vtable < self.vtable_bytes_len(&buf) {
            <VOffset>::read(&buf, pos_in_vtable)
        } else {
            0
        }
    }
}

impl Read for VTablePosition {
    /// VTablePosition is stored as a SOffset in the buffer.
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        VTablePosition(seek_soffset(pos, <SOffset>::read(&buf, pos)))
    }
}

impl AsRef<usize> for VTablePosition {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

/// TablePosition wrappers a position which points to a table in the buffer.
///
/// They start with an soffset_t to a vtable. This is a signed version of uoffset_t, since vtables
/// may be stored anywhere relative to the object. This offset is substracted (not added) from the
/// object start to arrive at the vtable start. This offset is followed by all the fields as
/// aligned scalars (or offsets). Unlike structs, not all fields need to be present. There is no
/// set order and layout.
///
/// To be able to access fields regardless of these uncertainties, we go through a vtable of
/// offsets. Vtables are shared between any objects that happen to have the same vtable values.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct TablePosition(usize);

impl TablePosition {
    pub fn new(pos: usize) -> TablePosition {
        TablePosition(pos)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn position(self) -> usize {
        self.0
    }

    /// Seeks the vtable position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::TablePosition;
    /// //         | -4               | vtable      | 4         |
    /// let buf = &[252, 255, 255, 255, 4u8, 0, 4, 0, 4, 0, 0, 0][..];
    ///
    /// let pos1 = TablePosition::new(0);
    /// assert_eq!(4, pos1.vtable(&buf).into_inner());
    /// let pos2 = TablePosition::new(8);
    /// assert_eq!(4, pos2.vtable(&buf).into_inner());
    /// ```
    pub fn vtable<T: AsRef<[u8]>>(self, buf: &T) -> VTablePosition {
        <VTablePosition>::read(&buf, self.0)
    }

    /// Seeks the position for a field.
    ///
    /// The field index is specified using `pos_in_vtable`, which is the offset inside vtable
    /// bytes. For example, 4 means the first field, 6 is the second.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::TablePosition;
    /// //       [vtable 10|    40|    20|    0|    4] [table   10]
    /// let buf = &[10u8, 0, 40, 0, 20, 0, 0, 0, 4, 0, 10, 0, 0, 0][..];
    /// let pos = TablePosition::new(10);
    ///
    /// assert_eq!(Some(20 + 10), pos.field_position(&buf, 4));
    /// assert_eq!(None, pos.field_position(&buf, 6));
    /// assert_eq!(Some(4 + 10), pos.field_position(&buf, 8));
    /// assert_eq!(None, pos.field_position(&buf, 10));
    /// ```
    pub fn field_position<T: AsRef<[u8]>>(self, buf: &T, pos_in_vtable: usize) -> Option<usize> {
        let vtable = self.vtable(&buf);
        let offset = vtable.field_offset(&buf, pos_in_vtable);
        if offset != 0 {
            Some(self.0 + offset as usize)
        } else {
            None
        }
    }
}

impl Read for TablePosition {
    /// TablePosition is stored as a UOffset in the buffer.
    fn read<T: AsRef<[u8]>>(buf: &T, pos: usize) -> Self {
        TablePosition(pos + <UOffset>::read(&buf, pos) as usize)
    }
}

impl AsRef<usize> for TablePosition {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}
