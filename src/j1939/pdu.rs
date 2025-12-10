#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PDU {
    format: u8,
    specific: u8,
}

impl PDU {
    pub fn new(format: u8, specific: u8) -> Self {
        Self { format, specific }
    }

    pub fn raw(&self) -> u16 {
        (self.get_format_raw() as u16) << 8 | (self.get_specific_raw() as u16)
    }

    pub fn get_format_raw(&self) -> u8 {
        self.format
    }

    pub fn get_specific_raw(&self) -> u8 {
        self.specific
    }

    pub fn get_type(&self) -> PDUType {
        PDUType::from(self.get_format_raw())
    }

    pub fn get_destination_address(&self) -> Option<u8> {
        matches!(self.get_type(), PDUType::PDU1).then_some(self.get_specific_raw())
    }

    pub fn get_group_extension(&self) -> Option<u8> {
        matches!(self.get_type(), PDUType::PDU2).then_some(self.get_specific_raw())
    }
}

impl From<u16> for PDU {
    fn from(value: u16) -> Self {
        Self::new(((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
    }
}

pub enum PDUType {
    PDU1,
    PDU2,
}

impl From<u8> for PDUType {
    fn from(value: u8) -> Self {
        if value < 0xF0 {
            PDUType::PDU1
        } else {
            PDUType::PDU2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdu1() {
        let pdu1 = PDU::new(0x11, 0x11);
        assert_eq!(pdu1.get_format_raw(), 0x11);
        assert_eq!(pdu1.get_specific_raw(), 0x11);
        assert_eq!(pdu1.raw(), 0x1111);
        assert!(matches!(pdu1.get_destination_address(), Some(0x11)));
        assert!(pdu1.get_group_extension().is_none());

        assert_eq!(pdu1, PDU::from(0x1111));
    }

    #[test]
    fn test_pdu2() {
        let pdu2 = PDU::new(0xF1, 0x11);
        assert_eq!(pdu2.get_format_raw(), 0xF1);
        assert_eq!(pdu2.get_specific_raw(), 0x11);
        assert_eq!(pdu2.raw(), 0xF111);
        assert!(pdu2.get_destination_address().is_none());
        assert!(matches!(pdu2.get_group_extension(), Some(0x11)));

        assert_eq!(pdu2, PDU::from(0xF111));
    }
}
