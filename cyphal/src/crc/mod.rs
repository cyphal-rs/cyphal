mod crc_table;
use crc_table::CRC_TABLE;

mod u64le;
use u64le::U64Le;

use core::ptr::NonNull;
use core::{cmp, slice};

#[inline]
pub fn crc32c(data: &[u8]) -> u32 {
    crc32c_append(0, data)
}

pub fn crc32c_append(crc: u32, buffer: &[u8]) -> u32 {
    let mut crc = u64::from(!crc);

    let (start, mid, end) = split(buffer);

    crc = crc_u8(crc, start);
    crc = crc_u64(crc, mid);
    crc = crc_u8(crc, end);

    !(crc as u32)
}

#[inline]
fn crc_u8(crc: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(crc, |crc, &next| {
        let index = (crc ^ u64::from(next)) as u8;
        CRC_TABLE.at(0, index) ^ (crc >> 8)
    })
}

#[inline]
fn crc_u64(crci: u64, buffer: &[U64Le]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let crc = crc ^ next.get();

        CRC_TABLE.at(7, crc as u8)
            ^ CRC_TABLE.at(6, (crc >> 8) as u8)
            ^ CRC_TABLE.at(5, (crc >> 16) as u8)
            ^ CRC_TABLE.at(4, (crc >> 24) as u8)
            ^ CRC_TABLE.at(3, (crc >> 32) as u8)
            ^ CRC_TABLE.at(2, (crc >> 40) as u8)
            ^ CRC_TABLE.at(1, (crc >> 48) as u8)
            ^ CRC_TABLE.at(0, (crc >> 56) as u8)
    })
}

/// Splits a buffer into three subslices:
/// - the first one is up to the first 8-byte aligned address.
/// - the second one is 8-byte aligned and its length is a multiple of 8.
/// - the third one is 8-byte aligned but its length is less than 8.
fn split(buffer: &[u8]) -> (&[u8], &[U64Le], &[u8]) {
    let (start, mid) = {
        let split_index = {
            let addr = buffer.as_ptr() as usize;

            // Align to multiples of 8.
            let aligned_addr = (addr + 7) & (!7);

            // Index of the next aligned element.
            let i = aligned_addr - addr;

            // Buffer might be too small.
            cmp::min(i, buffer.len())
        };

        buffer.split_at(split_index)
    };

    let (mid, end) = {
        // Round length down to multiples of 8.
        let split_index = mid.len() & (!7);

        mid.split_at(split_index)
    };

    let mid = unsafe {
        let length = mid.len() / 8;
        let ptr = if length == 0 {
            // `slice::from_raw_parts` requires that pointers be nonnull and
            // aligned even for zero-length slices.
            NonNull::<U64Le>::dangling().as_ptr()
        } else {
            #[allow(clippy::cast_ptr_alignment)]
            mid.as_ptr().cast::<U64Le>()
        };

        slice::from_raw_parts(ptr, length)
    };

    (start, mid, end)
}
