use le::LE;
use types::{SOffset, UOffset};

/// Reads a `SOffset` from `buf` at `pos`. Returns a new position by subtracting the read `SOffset`
/// from `pos`.
///
/// # Examples
///
/// ```
/// use fbg::seek::seek_soffset;
///
/// assert_eq!(0, seek_soffset(&[0u8, 1, 0, 0, 0], 1));
/// assert_eq!(4, seek_soffset(&[252, 255, 255, 255], 0));
/// ```
pub fn seek_soffset(buf: &[u8], pos: usize) -> usize {
    let offset = SOffset::from_le_slice(&buf[pos..]);
    pos.wrapping_sub(offset as usize)
}

/// Reads a `UOffset` from `buf` at `pos`. Returns a new position by adding the read `UOffset` to
/// `pos`.
///
/// # Examples
///
/// ```
/// use fbg::seek::seek_uoffset;
///
/// assert_eq!(5, seek_uoffset(&[0u8, 4, 0, 0, 0], 1));
/// ```
pub fn seek_uoffset(buf: &[u8], pos: usize) -> usize {
    let offset = UOffset::from_le_slice(&buf[pos..]);
    pos + offset as usize
}
