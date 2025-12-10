use std::{error::Error, fmt::Display};

use ux::{u3, u18, u29};

use crate::{can::can_id::CANID, j1939::pgn::PGN};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct J1939ID {
    priority: u3,
    pgn: PGN,
    source_address: u8,
}

impl J1939ID {
    pub fn new(priority: u3, pgn: PGN, source_address: u8) -> Self {
        Self {
            priority,
            pgn,
            source_address,
        }
    }

    pub fn raw(&self) -> u29 {
        (u29::from(self.priority) << 26)
            | (u29::from(self.pgn.raw()) << 8)
            | u29::from(self.source_address)
    }

    pub fn get_priority_raw(&self) -> u3 {
        self.priority
    }

    pub fn get_pgn(&self) -> PGN {
        self.pgn
    }

    pub fn get_source_address_raw(&self) -> u8 {
        self.source_address
    }
}

impl From<u29> for J1939ID {
    fn from(value: u29) -> Self {
        Self::new(
            u3::try_from((value & u29::new(0x1C000000)) >> 26).unwrap(),
            PGN::from(u18::try_from((value & u29::new(0xFFFF00)) >> 8).unwrap()),
            u8::try_from(value & u29::new(0xFF)).unwrap(),
        )
    }
}

impl TryFrom<CANID> for J1939ID {
    type Error = CANIDToJ1939IDError;

    fn try_from(value: CANID) -> Result<Self, Self::Error> {
        match value {
            CANID::Standard(_) => Err(CANIDToJ1939IDError),
            CANID::Extended(value) => Ok(J1939ID::from(value)),
        }
    }
}

#[derive(Debug)]
pub struct CANIDToJ1939IDError;

impl Display for CANIDToJ1939IDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to convert standard 11-bit CAN ID into 29-bit J1939 ID")
    }
}

impl Error for CANIDToJ1939IDError {}

#[cfg(test)]
mod tests {
    use crate::j1939::pdu::PDU;

    use super::*;

    #[test]
    fn test_j1939_id() {
        let j1939_id = J1939ID::new(u3::new(7), PGN::new(PDU::new(0x11, 0x11)), 0x11);
        assert_eq!(j1939_id.get_priority_raw(), u3::new(7));
        assert_eq!(j1939_id.get_source_address_raw(), 0x11);
        assert_eq!(j1939_id.raw(), u29::new(0x1C111111));

        assert_eq!(j1939_id, J1939ID::from(u29::new(0x1C111111)));
    }

    #[test]
    pub fn test_j1939_id_to_can_id() {
        let j1939_id = J1939ID::new(u3::new(6), PGN::new(PDU::new(0xFF, 0x01)), 5);

        let can_id = CANID::from(j1939_id);
        assert!(matches!(can_id, CANID::Extended(ref inner) if *inner == u29::new(0x18FF0105)));
    }
}
