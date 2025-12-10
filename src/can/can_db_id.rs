use ux::u11;

use crate::can::can_id::CANID;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CANDBID {
    Standard(u11),
    Extended(u32),
}

impl From<CANID> for CANDBID {
    fn from(value: CANID) -> Self {
        match value {
            CANID::Standard(value) => Self::Standard(value),
            CANID::Extended(value) => Self::Extended(u32::from(value) | (1 << 31)),
        }
    }
}

#[cfg(test)]
mod tests {
    use ux::{u11, u29};

    use super::*;

    #[test]
    fn test_can_db_id_to_can_id() {
        let standard_can_db_id = CANDBID::Standard(u11::new(0b00000000001));
        let standard_can_id = CANID::from(standard_can_db_id);
        assert_eq!(standard_can_id, CANID::Standard(u11::new(0b00000000001)));

        let extended_can_db_id = CANDBID::Extended(0b10000000000000000000000000000001);
        let extended_can_id = CANID::from(extended_can_db_id);
        assert_eq!(extended_can_id, CANID::Extended(u29::new(0b00000000000000000000000000001)));
    }
}
