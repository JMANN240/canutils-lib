use ux::{u11, u29};

use crate::{can::can_db_id::CANDBID, j1939::j1939_id::J1939ID};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CANID {
    Standard(u11),
    Extended(u29),
}

impl From<J1939ID> for CANID {
    fn from(value: J1939ID) -> Self {
        Self::Extended(value.raw())
    }
}

impl From<CANDBID> for CANID {
    fn from(value: CANDBID) -> Self {
        match value {
            CANDBID::Standard(value) => Self::Standard(value),
            CANDBID::Extended(value) => Self::Extended(u29::try_from(value & 0x1FFFFFFF).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use ux::{u3, u11};

    use crate::j1939::{pdu::PDU, pgn::PGN};

    use super::*;

    #[test]
    fn test_can_id_to_can_db_id() {
        let standard_can_id = CANID::Standard(u11::new(0b00000000001));
        let standard_can_db_id = CANDBID::from(standard_can_id);
        assert_eq!(standard_can_db_id, CANDBID::Standard(u11::new(0b00000000001)));

        let extended_can_id = CANID::Extended(u29::new(0b00000000000000000000000000001));
        let extended_can_db_id = CANDBID::from(extended_can_id);
        assert_eq!(extended_can_db_id, CANDBID::Extended(0b10000000000000000000000000000001));
    }

    #[test]
    pub fn test_can_id_to_j1939_id() {
        let standard_can_id = CANID::Standard(u11::new(0b00000000000));
        assert_err!(J1939ID::try_from(standard_can_id));

        let extended_can_id = CANID::Extended(u29::new(0x18FF0105));
        let j1939_id = assert_ok!(J1939ID::try_from(extended_can_id));
        assert_eq!(j1939_id.get_priority_raw(), u3::new(6));
        assert_eq!(j1939_id.get_pgn(), PGN::new(PDU::new(0xFF, 0x01)));
        assert_eq!(j1939_id.get_source_address_raw(), 5);
    }
}
