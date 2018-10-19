use io::Read;
use position::{TablePosition, VTablePosition};
use std::mem::size_of;
use types::VOffset;

/// Table wraps the buffer and the table position, so the table fields can be fetched without other
/// dependencies.
///
/// See usage in `TableWithVTable::read_field`
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Table<T> {
    buf: T,
    pos: TablePosition,
}

/// TableWithVTable does not only wrap the buffer and the table position, it also caches the vtable
/// position, which can speed up field lookup, since vtable position does not need to be read from
/// buffer each time.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct TableWithVTable<T> {
    table: Table<T>,
    vpos: VTablePosition,
}

impl<T> Table<T> {
    pub fn new(buf: T, pos: TablePosition) -> Table<T> {
        Table { buf, pos }
    }

    pub fn buffer(&self) -> &T {
        &self.buf
    }

    pub fn position(&self) -> &TablePosition {
        &self.pos
    }
}

impl<'a, T: Clone> Table<&'a T> {
    /// Clones the underlying buffer to create an owned table.
    pub fn into_owned(self) -> Table<T> {
        Table {
            buf: self.buf.clone(),
            pos: self.pos,
        }
    }
}

impl<'a, T: Clone> TableWithVTable<&'a T> {
    /// Clones the underlying buffer to create an owned table.
    pub fn into_owned(self) -> TableWithVTable<T> {
        TableWithVTable {
            table: self.table.into_owned(),
            vpos: self.vpos,
        }
    }
}

impl<T> TableWithVTable<T> {
    pub fn buffer(&self) -> &T {
        &self.table.buf
    }

    pub fn table(&self) -> &Table<T> {
        &self.table
    }

    pub fn position(&self) -> &VTablePosition {
        &self.vpos
    }
}

impl<T: AsRef<[u8]>> From<Table<T>> for TableWithVTable<T> {
    fn from(v: Table<T>) -> Self {
        let vpos = v.pos.vtable(&v.buf.as_ref());
        TableWithVTable { vpos, table: v }
    }
}

impl<T: AsRef<[u8]>> Table<T> {
    /// Reads the vtable position and creates TableWithVTable
    pub fn into_with_vtable(self) -> TableWithVTable<T> {
        self.into()
    }
}

impl<T: AsRef<[u8]>> TableWithVTable<T> {
    fn buf_bytes(&self) -> &[u8] {
        &self.table.buf.as_ref()
    }

    /// Reads the size of the vtable in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::{Table, position::TablePosition};
    ///
    /// let buf = &[4u8, 0, 6, 0, 4, 0, 0, 0][..];
    /// let table = Table::new(&buf, TablePosition::new(4)).into_with_vtable();
    ///
    /// assert_eq!(4, table.vtable_bytes_len());
    /// ```
    pub fn vtable_bytes_len(&self) -> usize {
        self.vpos.vtable_bytes_len(&self.buf_bytes())
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
    /// use fbg::{Table, position::TablePosition};
    ///
    /// // Field offsets are 20, 0, 4
    /// let buf = &[10u8, 0, 40, 0, 20, 0, 0, 0, 4, 0, 10, 0, 0, 0][..];
    /// let table = Table::new(&buf, TablePosition::new(10)).into_with_vtable();
    ///
    /// assert_eq!(20, table.field_offset(4));
    /// assert_eq!(0, table.field_offset(6));
    /// assert_eq!(4, table.field_offset(8));
    /// // Returns 0 when table_in_vtable is out of range.
    /// assert_eq!(0, table.field_offset(10));
    /// ```
    pub fn field_offset(&self, pos_in_vtable: usize) -> VOffset {
        self.vpos.field_offset(&self.buf_bytes(), pos_in_vtable)
    }

    /// Seeks the position for a field.
    ///
    /// The field index is specified using `pos_in_vtable`, which is the offset inside vtable
    /// bytes. For example, 4 means the first field, 6 is the second.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::{Table, position::TablePosition};
    /// //       [vtable 10|    40|    20|    0|    4] [table   10]
    /// let buf = &[10u8, 0, 40, 0, 20, 0, 0, 0, 4, 0, 10, 0, 0, 0][..];
    /// let table = Table::new(&buf, TablePosition::new(10)).into_with_vtable();
    ///
    /// assert_eq!(Some(20 + 10), table.field_position(4));
    /// assert_eq!(None, table.field_position(6));
    /// assert_eq!(Some(4 + 10), table.field_position(8));
    /// assert_eq!(None, table.field_position(10));
    /// ```
    pub fn field_position(&self, pos_in_vtable: usize) -> Option<usize> {
        let offset = self.field_offset(pos_in_vtable);
        if offset != 0 {
            Some(self.table.pos.position() + offset as usize)
        } else {
            None
        }
    }

    /// Reads a scalar field from buffer and convert it to native endian.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::{Table, position::TablePosition};
    /// //       [vtable 6|    6|    4] [table   6|,   1]
    /// let buf = &[6u8, 0, 6, 0, 4, 0, 6, 0, 0, 0, 1, 0][..];
    /// let table = Table::new(&buf, TablePosition::new(6)).into_with_vtable();
    ///
    /// assert_eq!(Some(1), table.read_field::<u16>(4));
    /// assert_eq!(None, table.read_field::<u16>(6));
    /// ```
    pub fn read_field<F: Read>(&self, pos_in_vtable: usize) -> Option<F> {
        self.field_position(pos_in_vtable)
            .map(|pos| <F>::read(&self.buf_bytes(), pos))
    }

    /// Gets reference to a field by directly casting the pointer into the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbg::{Table, position::TablePosition, Scalar};
    /// //       [vtable 6|    6|    4] [table   6|,   1]
    /// let buf = &[6u8, 0, 6, 0, 4, 0, 6, 0, 0, 0, 1, 0][..];
    /// let table = Table::new(&buf, TablePosition::new(6)).into_with_vtable();
    ///
    /// #[repr(C, align(1))]
    /// #[derive(Debug, PartialOrd, PartialEq)]
    /// struct Wrapper {
    ///     pub inner: Scalar<u16>
    /// }
    ///
    /// assert_eq!(1u16, table.cast_field_ref::<Wrapper>(4).unwrap().inner.into());
    /// assert_eq!(None, table.cast_field_ref::<Wrapper>(6));
    /// ```
    pub fn cast_field_ref<F: Sized>(&self, pos_in_vtable: usize) -> Option<&F> {
        self.field_position(pos_in_vtable).map(|pos| {
            let buf = &self.buf_bytes()[pos..pos + size_of::<F>()];
            let ptr = buf.as_ptr() as *const F;
            unsafe { &*ptr }
        })
    }

    /// Returns the field bytes directly.
    ///
    /// The bytes are in little endian form.
    pub fn field_slice<F: Sized>(&self, pos_in_vtable: usize) -> Option<&[u8]> {
        self.field_position(pos_in_vtable)
            .map(|pos| &self.buf_bytes()[pos..pos + size_of::<F>()])
    }
}
