// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Various bitfield constants of Section 6.3 of ETSI TS 103 636-4 V2.1.1

/// Version field of the MAC Header type
///
/// The standard only allows this value.
pub const VERSION: u8 = 0;

/// Values of the MAC Header Security field
///
/// See Table 6.3.2-1
pub mod security {
    pub const NOTUSED: u8 = 0;
    pub const USED_NO_IE: u8 = 1;
    pub const USED_WITH_IE: u8 = 2;
}

/// Values of the MAC Header Type field
///
/// See Table 6.3.2-2
pub mod header_type {
    pub const DATA_MAC_PDU: u8 = 0x0;
    pub const BEACON: u8 = 0x1;
    pub const UNICAST: u8 = 0x2;
    pub const RD_BROADCAST: u8 = 0x3;
    pub const ESCAPE: u8 = 0xf;
}

/// Values of tha MAC Extension field
///
/// See Table 6.3.4-1
pub mod mux_ext {
    pub const NO_LENGTH_FIELD: u8 = 0b00;
    pub const LENGTH_8BIT: u8 = 0b01;
    pub const LENGTH_16BIT: u8 = 0b10;
    pub const SHORT_IE: u8 = 0b11;
}
