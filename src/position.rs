use le::LE;
use seek::seek_soffset;
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;
use types::{Len, VOffset, SIZE_OF_LEN, SIZE_OF_VOFFSET};

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
/// let pos = VectorPosition(0);
///
/// assert_eq!(2, pos.len(buf));
/// assert_eq!(&[1u16.to_le(), 2u16.to_le()], pos.as_slice::<u16>(buf));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct VectorPosition(pub usize);

impl VectorPosition {
    /// Reads the length of the vector.
    pub fn len(self, buf: &[u8]) -> usize {
        Len::from_le_slice(buf) as usize
    }

    /// Gets the reference to the items slice.
    ///
    /// The slice attaches to the buffer directly, so all scalars are in little endian form.
    pub fn as_slice<T>(self, buf: &[u8]) -> &[T] {
        let len = self.len(buf);
        let start_pos = self.0 + SIZE_OF_LEN;
        let end_pos = start_pos + len * size_of::<T>();
        let ptr = (&buf[start_pos..end_pos]).as_ptr() as *const T;

        unsafe { from_raw_parts(ptr, len) }
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
/// let pos = StringPosition(0);
///
/// assert_eq!(3, pos.len(buf));
/// assert_eq!("fbg", pos.as_str(buf));
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct StringPosition(pub usize);

impl StringPosition {
    /// Reads the length of the string in bytes.
    pub fn len(self, buf: &[u8]) -> usize {
        Len::from_le_slice(buf) as usize
    }

    /// Gets the reference to the string.
    pub fn as_str(self, buf: &[u8]) -> &str {
        let len = self.len(buf);
        let start_pos = self.0 + SIZE_OF_LEN;
        let end_pos = start_pos + len;

        unsafe { from_utf8_unchecked(&buf[start_pos..end_pos]) }
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
pub struct VTablePosition(pub usize);

impl VTablePosition {
    /// Reads the size of the vtable in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use fbg::position::VTablePosition;
    ///
    /// let buf = &[4u8, 0, 6, 0][..];
    /// let pos = VTablePosition(0);
    ///
    /// assert_eq!(4, pos.vtable_bytes_len(&buf));
    /// ```
    pub fn vtable_bytes_len(self, buf: &[u8]) -> usize {
        VOffset::from_le_slice(&buf[self.0..]) as usize
    }

    /// Reads the size of the table in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::VTablePosition;
    ///
    /// let buf = &[4u8, 0, 6, 0][..];
    /// let pos = VTablePosition(0);
    ///
    /// assert_eq!(6, pos.table_bytes_len(&buf));
    /// ```
    pub fn table_bytes_len(self, buf: &[u8]) -> usize {
        VOffset::from_le_slice(&buf[self.0 + SIZE_OF_VOFFSET..]) as usize
    }

    /// Reads the field offset.
    ///
    /// Parameter `voffset_offset` is the position of the field voffset inside vtable bytes. For
    /// example 4 means the offset for the first field in the schema.
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
    /// let pos = VTablePosition(0);
    ///
    /// assert_eq!(20, pos.field_offset(&buf, 4));
    /// assert_eq!(0, pos.field_offset(&buf, 6));
    /// assert_eq!(4, pos.field_offset(&buf, 8));
    /// // Returns 0 when voffset_offset is out of range.
    /// assert_eq!(0, pos.field_offset(&buf, 10));
    /// ```
    pub fn field_offset(self, buf: &[u8], voffset_offset: usize) -> VOffset {
        if voffset_offset < self.vtable_bytes_len(&buf) {
            VOffset::from_le_slice(&buf[voffset_offset..])
        } else {
            0
        }
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
pub struct TablePosition(pub usize);

impl TablePosition {
    /// Seeks the vtable position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::position::TablePosition;
    /// //         | -4               | vtable      | 4         |
    /// let buf = &[252, 255, 255, 255, 4u8, 0, 4, 0, 4, 0, 0, 0][..];
    ///
    /// let pos1 = TablePosition(0);
    /// assert_eq!(4, pos1.vtable(&buf).0);
    /// let pos2 = TablePosition(8);
    /// assert_eq!(4, pos2.vtable(&buf).0);
    /// ```
    pub fn vtable(self, buf: &[u8]) -> VTablePosition {
        VTablePosition(seek_soffset(buf, self.0))
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
    /// let pos = TablePosition(10);
    ///
    /// assert_eq!(Some(20 + 10), pos.field_position(&buf, 4));
    /// assert_eq!(None, pos.field_position(&buf, 6));
    /// assert_eq!(Some(4 + 10), pos.field_position(&buf, 8));
    /// assert_eq!(None, pos.field_position(&buf, 10));
    /// ```
    pub fn field_position(self, buf: &[u8], pos_in_vtable: usize) -> Option<usize> {
        let vtable = self.vtable(&buf);
        let offset = vtable.field_offset(&buf, pos_in_vtable);
        if offset != 0 {
            Some(self.0 + offset as usize)
        } else {
            None
        }
    }
}
