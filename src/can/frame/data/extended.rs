use bitvec::prelude::*;
use ux::{u1, u3, u4, u7, u11, u15, u18, u29};

use crate::{can::frame::CANFrameDecodingError, unstuff};

pub struct ExtendedDataFrame {
    start_of_frame: u1,
    identifier_a: u11,
    substitute_remote_request: u1,
    identifier_extension_bit: u1,
    identifier_b: u18,
    remote_transmission_request: u1,
    reserved_bit_one: u1,
    reserved_bit_zero: u1,
    data_length_code: u4,
    data_field: Vec<u8>,
    cyclic_redundancy_check: u15,
    cyclic_redundancy_check_delimiter: u1,
    acknowledgement_slot: u1,
    acknowledgement_delimiter: u1,
    end_of_frame: u7,
    inter_frame_spacing: u3,
}

impl ExtendedDataFrame {
    pub fn from_unstuffed_bits<T: BitStore, B: AsRef<BitSlice<T, Msb0>>>(
        unstuffed_bits: B,
    ) -> Result<Self, CANFrameDecodingError> {
        let unstuffed_bits = unstuffed_bits.as_ref();

        let mut bit_index = 0;

        let maybe_start_of_frame_bit_ref = unstuffed_bits.get(bit_index);
        let start_of_frame_bit_ref =
            maybe_start_of_frame_bit_ref.ok_or(CANFrameDecodingError::StartOfFrameMissing)?;
        let start_of_frame = u1::from(*start_of_frame_bit_ref);
        if start_of_frame != u1::new(0) {
            return Err(CANFrameDecodingError::StartOfFrameMustBeZero);
        }

        bit_index += 1;

        let maybe_identifier_a_bit_slice = unstuffed_bits.get(bit_index..(bit_index + 11));
        let identifier_a_bit_slice =
            maybe_identifier_a_bit_slice.ok_or(CANFrameDecodingError::IdentifierMissing)?;
        let identifier_a = u11::new(identifier_a_bit_slice.load());

        bit_index += 11;

        let maybe_substitute_remote_request_bit_ref = unstuffed_bits.get(bit_index);
        let substitute_remote_request_bit_ref = maybe_substitute_remote_request_bit_ref
            .ok_or(CANFrameDecodingError::SubstituteRemoteRequestMissing)?;
        let substitute_remote_request = u1::from(*substitute_remote_request_bit_ref);
        if substitute_remote_request != u1::new(1) {
            return Err(CANFrameDecodingError::SubstituteRemoteRequestMustBeOne);
        }

        bit_index += 1;

        let maybe_identifier_extension_bit_bit_ref = unstuffed_bits.get(bit_index);
        let identifier_extension_bit_bit_ref = maybe_identifier_extension_bit_bit_ref
            .ok_or(CANFrameDecodingError::IdentifierExtensionBitMissing)?;
        let identifier_extension_bit = u1::from(*identifier_extension_bit_bit_ref);
        if identifier_extension_bit != u1::new(1) {
            return Err(CANFrameDecodingError::IdentifierExtensionBitMustBeOne);
        }

        bit_index += 1;

        let maybe_identifier_b_bit_slice = unstuffed_bits.get(bit_index..(bit_index + 18));
        let identifier_b_bit_slice =
            maybe_identifier_b_bit_slice.ok_or(CANFrameDecodingError::IdentifierMissing)?;
        let identifier_b = u18::new(identifier_b_bit_slice.load());

        bit_index += 18;

        let maybe_remote_transmission_request_bit_ref = unstuffed_bits.get(bit_index);
        let remote_transmission_request_bit_ref = maybe_remote_transmission_request_bit_ref
            .ok_or(CANFrameDecodingError::RemoteTransmissionRequestMissing)?;
        let remote_transmission_request = u1::from(*remote_transmission_request_bit_ref);

        bit_index += 1;

        let maybe_reserved_bit_one_bit_ref = unstuffed_bits.get(bit_index);
        let reserved_bit_one_bit_ref =
            maybe_reserved_bit_one_bit_ref.ok_or(CANFrameDecodingError::ReservedBitOneMissing)?;
        let reserved_bit_one = u1::from(*reserved_bit_one_bit_ref);

        bit_index += 1;

        let maybe_reserved_bit_zero_bit_ref = unstuffed_bits.get(bit_index);
        let reserved_bit_zero_bit_ref =
            maybe_reserved_bit_zero_bit_ref.ok_or(CANFrameDecodingError::ReservedBitZeroMissing)?;
        let reserved_bit_zero = u1::from(*reserved_bit_zero_bit_ref);

        bit_index += 1;

        let maybe_data_length_code_bit_slice = unstuffed_bits.get(bit_index..(bit_index + 4));
        let data_length_code_bit_slice =
            maybe_data_length_code_bit_slice.ok_or(CANFrameDecodingError::IdentifierMissing)?;
        let data_length_code = u4::new(data_length_code_bit_slice.load());

        bit_index += 4;

        let data_field = Vec::new();

        bit_index += 8 * u8::from(data_length_code) as usize;

        let maybe_cyclic_redundancy_check_bit_slice =
            unstuffed_bits.get(bit_index..(bit_index + 15));
        let cyclic_redundancy_check_bit_slice = maybe_cyclic_redundancy_check_bit_slice
            .ok_or(CANFrameDecodingError::CyclicRedundancyCheckMissing)?;
        let cyclic_redundancy_check = u15::new(cyclic_redundancy_check_bit_slice.load());

        bit_index += 15;

        let maybe_cyclic_redundancy_check_delimiter_bit_ref = unstuffed_bits.get(bit_index);
        let cyclic_redundancy_check_delimiter_bit_ref =
            maybe_cyclic_redundancy_check_delimiter_bit_ref
                .ok_or(CANFrameDecodingError::CyclicRedundancyCheckDelimiterMissing)?;
        let cyclic_redundancy_check_delimiter =
            u1::from(*cyclic_redundancy_check_delimiter_bit_ref);
        if cyclic_redundancy_check_delimiter != u1::new(1) {
            return Err(CANFrameDecodingError::CyclicRedundancyCheckDelimiterMustBeOne);
        }

        bit_index += 1;

        let maybe_acknowledgement_slot_bit_ref = unstuffed_bits.get(bit_index);
        let acknowledgement_slot_bit_ref = maybe_acknowledgement_slot_bit_ref
            .ok_or(CANFrameDecodingError::AcknowledgementSlotMissing)?;
        let acknowledgement_slot = u1::from(*acknowledgement_slot_bit_ref);

        bit_index += 1;

        let maybe_acknowledgement_delimiter_bit_ref = unstuffed_bits.get(bit_index);
        let acknowledgement_delimiter_bit_ref = maybe_acknowledgement_delimiter_bit_ref
            .ok_or(CANFrameDecodingError::AcknowledgementDelimiterMissing)?;
        let acknowledgement_delimiter = u1::from(*acknowledgement_delimiter_bit_ref);
        if acknowledgement_delimiter != u1::new(1) {
            return Err(CANFrameDecodingError::AcknowledgementDelimiterMustBeOne);
        }

        bit_index += 1;

        let maybe_end_of_frame_bit_slice = unstuffed_bits.get(bit_index..(bit_index + 7));
        let end_of_frame_bit_slice =
            maybe_end_of_frame_bit_slice.ok_or(CANFrameDecodingError::EndOfFrameMissing)?;
        let end_of_frame = u7::new(end_of_frame_bit_slice.load());
        if end_of_frame != u7::new(0b1111111) {
            return Err(CANFrameDecodingError::EndOfFrameMustBeOne);
        }

        bit_index += 7;

        let maybe_inter_frame_spacing_bit_slice = unstuffed_bits.get(bit_index..(bit_index + 3));
        let inter_frame_spacing_bit_slice = maybe_inter_frame_spacing_bit_slice
            .ok_or(CANFrameDecodingError::InterFrameSpacingMissing)?;
        let inter_frame_spacing = u3::new(inter_frame_spacing_bit_slice.load());
        if inter_frame_spacing != u3::new(0b111) {
            return Err(CANFrameDecodingError::InterFrameSpacingMustBeOne);
        }

        Ok(Self {
            start_of_frame,
            identifier_a,
            substitute_remote_request,
            identifier_extension_bit,
            identifier_b,
            remote_transmission_request,
            reserved_bit_one,
            reserved_bit_zero,
            data_length_code,
            data_field,
            cyclic_redundancy_check,
            cyclic_redundancy_check_delimiter,
            acknowledgement_slot,
            acknowledgement_delimiter,
            end_of_frame,
            inter_frame_spacing,
        })
    }

    pub fn from_stuffed_bits<T: BitStore, B: AsRef<BitSlice<T, Msb0>>>(
        stuffed_bits: B,
    ) -> Result<Self, CANFrameDecodingError> {
        Self::from_unstuffed_bits(unstuff(stuffed_bits, 5))
    }

    pub fn start_of_frame(&self) -> u1 {
        self.start_of_frame
    }

    pub fn identifier_a(&self) -> u11 {
        self.identifier_a
    }

    pub fn substitute_remote_request(&self) -> u1 {
        self.substitute_remote_request
    }

    pub fn identifier_extension_bit(&self) -> u1 {
        self.identifier_extension_bit
    }

    pub fn identifier_b(&self) -> u18 {
        self.identifier_b
    }

    pub fn identifier(&self) -> u29 {
        (u29::from(self.identifier_a()) << 18) | u29::from(self.identifier_b())
    }

    pub fn remote_transmission_request(&self) -> u1 {
        self.remote_transmission_request
    }

    pub fn reserved_bit_one(&self) -> u1 {
        self.reserved_bit_one
    }

    pub fn reserved_bit_zero(&self) -> u1 {
        self.reserved_bit_zero
    }

    pub fn data_length_code(&self) -> u4 {
        self.data_length_code
    }

    pub fn data_field(&self) -> &Vec<u8> {
        &self.data_field
    }

    pub fn cyclic_redundancy_check(&self) -> u15 {
        self.cyclic_redundancy_check
    }

    pub fn cyclic_redundancy_check_delimiter(&self) -> u1 {
        self.cyclic_redundancy_check_delimiter
    }

    pub fn acknowledgement_slot(&self) -> u1 {
        self.acknowledgement_slot
    }

    pub fn acknowledgement_delimiter(&self) -> u1 {
        self.acknowledgement_delimiter
    }

    pub fn end_of_frame(&self) -> u7 {
        self.end_of_frame
    }

    pub fn inter_frame_spacing(&self) -> u3 {
        self.inter_frame_spacing
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[test]
    fn test_from_stuffed_bits() {
        let extended_data_frame =
            assert_ok!(ExtendedDataFrame::from_stuffed_bits(bitvec![usize, Msb0;
                0,                                                      // SOF
                0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0,                     // ID A
                1,                                                      // SRR
                1,                                                      // IDE
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,   // ID B
                0,                                                      // RTR
                0,                                                      // R1
                0,                                                      // R0
                0, 1, 0, 0, 1,                                          // DLC
                0, 0, 0, 0, 0, 1, 0, 0, 1,                              // DF
                1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1,            // CRC
                1,                                                      // CRC Delim
                0,                                                      // ACK Slot
                1,                                                      // ACK Delim
                1, 1, 1, 1, 1, 1, 1,                                    // EOF
                1, 1, 1,                                                // IFS
            ],));

        assert_eq!(extended_data_frame.start_of_frame(), u1::new(0b0));
        assert_eq!(extended_data_frame.identifier_a(), u11::new(0b00000010100));
        assert_eq!(extended_data_frame.substitute_remote_request(), u1::new(0b1));
        assert_eq!(extended_data_frame.identifier_extension_bit(), u1::new(0b1));
        assert_eq!(extended_data_frame.identifier_b(), u18::new(0b101010101010101010));
        assert_eq!(extended_data_frame.identifier(), u29::new(0b00000010100101010101010101010));
        assert_eq!(
            extended_data_frame.remote_transmission_request(),
            u1::new(0b0)
        );
        assert_eq!(extended_data_frame.reserved_bit_one(), u1::new(0b0));
        assert_eq!(extended_data_frame.reserved_bit_zero(), u1::new(0b0));
        assert_eq!(extended_data_frame.data_length_code(), u4::new(0b0001));
        assert_eq!(
            extended_data_frame.cyclic_redundancy_check(),
            u15::new(0b111011101010011)
        );
        assert_eq!(
            extended_data_frame.cyclic_redundancy_check_delimiter(),
            u1::new(0b1)
        );
        assert_eq!(extended_data_frame.acknowledgement_slot(), u1::new(0b0));
        assert_eq!(
            extended_data_frame.acknowledgement_delimiter(),
            u1::new(0b1)
        );
        assert_eq!(extended_data_frame.end_of_frame(), u7::new(0b1111111));
        assert_eq!(extended_data_frame.inter_frame_spacing(), u3::new(0b111));
    }
}
