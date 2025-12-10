use ux::{u1, u18};

use crate::j1939::pdu::PDU;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PGN {
    reserved: u1,
    data_page: u1,
    pdu: PDU,
}

impl PGN {
    pub fn new(pdu: PDU) -> Self {
        Self {
            reserved: u1::new(0),
            data_page: u1::new(0),
            pdu,
        }
    }

    pub fn raw(&self) -> u18 {
        (u18::from(self.reserved) << 17) | (u18::from(self.data_page) << 16) | u18::from(self.pdu.raw())
    }

    pub fn get_reserved_raw(&self) -> u1 {
        self.reserved
    }

    pub fn get_data_page_raw(&self) -> u1 {
        self.data_page
    }

    pub fn get_pdu(&self) -> PDU {
        self.pdu
    }
}

impl From<u18> for PGN {
    fn from(value: u18) -> Self {
        Self::new(PDU::from(u16::try_from(value & u18::new(0xFFFF)).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn() {
        let pgn = PGN::new(PDU::new(0x11, 0x11));
        assert_eq!(pgn.get_reserved_raw(), u1::new(0));
        assert_eq!(pgn.get_data_page_raw(), u1::new(0));
        assert_eq!(pgn.raw(), u18::new(0x1111));

        assert_eq!(pgn, PGN::from(u18::new(0x1111)));
    }
}
