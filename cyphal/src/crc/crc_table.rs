include!(concat!(env!("OUT_DIR"), "/crc_table_data.rs"));

/// 8-KiB lookup table.
pub(crate) struct CrcTable([[u32; 256]; 8]);

impl CrcTable {
    /// Returns an entry from the table.
    #[inline]
    pub fn at(&self, i: u8, j: u8) -> u64 {
        let i = i as usize;
        let j = j as usize;
        u64::from(self.0[i][j])
    }
}
