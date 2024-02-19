/// A newtype wrapper for a little endian `u64`.
///
/// It is safe to transmute between a `u64` and `U64Le`.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct U64Le(u64);

impl U64Le {
    /// Returns a `u64` with correct endianness for the target.
    ///
    /// On little endian targets, this is a no-op.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub const fn get(self) -> u64 {
        u64::from_le(self.0)
    }
}
