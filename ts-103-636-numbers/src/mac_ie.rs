//! IE (Information Element) type

/// An IE type as uses with MAC Extension field encodings 00/01/10
///
/// ## Invariants
///
/// The inner field only has the lowest 6 bit set. This is not a safety invariant: This module will
/// only either produce wrong results or panic if the upper bits are set.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct IEType6bit(u8);

impl IEType6bit {
    /// Definition of the element from Table 6.3.4-2
    ///
    /// Editorial liberty is used to convert remove "IE" and "message" suffixes.
    const fn description(&self) -> Option<&'static str> {
        Some(match self.0 {
            0b000000 => "Padding",
            0b000001 => "Higher layer signalling - flow 1",
            0b000010 => "Higher layer signalling - flow 2",
            0b000011 => "User plane data - flow 1",
            0b000100 => "User plane data - flow 2",
            0b000101 => "User plane data - flow 3",
            0b000110 => "User plane data - flow 4",
            0b001000 => "Network Beacon",
            0b001001 => "Cluster Beacon",
            0b001010 => "Association Request",
            0b001011 => "Association Response",
            0b001100 => "Association Release",
            0b001101 => "Reconfiguration Request",
            0b001110 => "Reconfiguration Response",
            0b001111 => "Additional MAC messages",
            0b010000 => "MAC Security Info",
            0b010001 => "Route Info",
            0b010010 => "Resource allocation",
            0b010011 => "Random Access Resource",
            0b010100 => "RD capability",
            0b010101 => "Neighbouring",
            0b010110 => "Broadcast Indication",
            0b010111 => "Group Assignment",
            0b011000 => "Load Info",
            0b011001 => "Measurement Report",
            0b011010 => "Source Routing",
            0b011011 => "Joining Beacon",
            0b011100 => "Joining Information",
            0b011110 => "Escape",
            0b011111 => "IE type extension",
            _ => return None,
        })
    }
}

impl core::fmt::Debug for IEType6bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debugtuple = f.debug_struct("IEType6bit");
        debugtuple.field(".0", &format_args!("{:#04x}", self.0));
        if let Some(description) = self.description() {
            debugtuple.field("description", &description);
        }
        debugtuple.finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for IEType6bit {
    fn format(&self, f: defmt::Formatter<'_>) {
        if let Some(description) = self.description() {
            defmt::write!(
                f,
                "IEType6bit {{ .0: {:#04x}, description: {} }}",
                self.0,
                description
            );
        } else {
            defmt::write!(
                f,
                "IEType6bit {{ .0: {:#04x} }}",
                self.0
            );
        }
    }
}

impl TryFrom<u8> for IEType6bit {
    type Error = super::ExcessiveBitsSet;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & !0x3f == 0 {
            Ok(Self(value))
        } else {
            Err(super::ExcessiveBitsSet)
        }
    }
}

impl From<IEType6bit> for u8 {
    #[inline]
    fn from(value: IEType6bit) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        IEType6bit::try_from(0xff).unwrap_err();
    }

    #[test]
    fn test_debug() {
        extern crate std;
        use std::format;

        assert_eq!(
            &format!("{:?}", IEType6bit::try_from(0b001000).unwrap()),
            &"IEType6bit { .0: 0x08, description: \"Network Beacon\" }"
        );

        assert_eq!(
            // reserved
            &format!("{:?}", IEType6bit::try_from(0b011101).unwrap()),
            &"IEType6bit { .0: 0x1d }"
        );
    }
}
