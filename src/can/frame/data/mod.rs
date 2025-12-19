use bitvec::prelude::*;

use crate::can::frame::CANFrameDecodingError;

pub mod base;
pub mod extended;

fn extract_field<U, T, B, F>(
    bits: B,
    offset: usize,
    length: usize,
    missing_error: CANFrameDecodingError,
    load: F,
) -> Result<U, CANFrameDecodingError>
where
    T: BitStore,
    B: AsRef<BitSlice<T, Msb0>>,
    F: Fn(&BitSlice<T, Msb0>) -> U
{
    let maybe_bit_slice = bits.as_ref().get(offset..(offset + length));
    let bit_slice = maybe_bit_slice.ok_or(missing_error)?;
    Ok(load(bit_slice))
}
