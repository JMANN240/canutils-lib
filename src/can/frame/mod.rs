use std::error::Error;

use strum::Display;

pub mod data;

#[derive(Display, Debug, Copy, Clone)]
pub enum CANFrameDecodingError {
    StartOfFrameMissing,
    StartOfFrameMustBeZero,
    IdentifierMissing,
    IdentifierAMissing,
    IdentifierBMissing,
    SubstituteRemoteRequestMissing,
    SubstituteRemoteRequestMustBeOne,
    RemoteTransmissionRequestMissing,
    IdentifierExtensionBitMissing,
    IdentifierExtensionBitMustBeZero,
    IdentifierExtensionBitMustBeOne,
    ReservedBitOneMissing,
    ReservedBitZeroMissing,
    CyclicRedundancyCheckMissing,
    CyclicRedundancyCheckDelimiterMissing,
    CyclicRedundancyCheckDelimiterMustBeOne,
    AcknowledgementSlotMissing,
    AcknowledgementDelimiterMissing,
    AcknowledgementDelimiterMustBeOne,
    EndOfFrameMissing,
    EndOfFrameMustBeOne,
    InterFrameSpacingMissing,
    InterFrameSpacingMustBeOne,
}

impl Error for CANFrameDecodingError {}
