//! Allocations in the [DECT-2020 NR Endpoint Multiplexing Address Allocation] mapping.
//!
//! [DECT-2020 NR Endpoint Multiplexing Address Allocation]: https://portal.etsi.org/PNNS/Protocol-Specification-Allocation/DECT-2020-NR-Endpoint-Multiplexing-Addresses
//!
//! The entries have no systematic or machine-suitable names. Therefore, names are assigned in this
//! crate, with the documentation containing the full name.

/// Newtype for endpoint multiplexing addresses
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct EndpointMultiplexingAddress(pub u16);

impl EndpointMultiplexingAddress {
    /// Description of the address
    ///
    /// Editorial liberty is used to convert names into consistent singular, normalize case, and
    /// remove "Identifier of" prefixes.
    const fn description(self) -> Option<&'static str> {
        Some(match self {
            // If this list gets longer, consider macros or code generation to keep all in sync
            DATAGRAM_IPV6 => "IPv6 datagram",
            DATAGRAM_6LO => "IPv6 datagram with header compression as defined in RFC6282",
            CONFIGURATION_DATA_REQUEST => "Configuration Data Request",
            CONFIGURATION_DATA_RESPONSE => "Configuration Data Response",
            _ => return None,
        })
    }
}

// Once feature(derive_from) has been stabilized, use that.
impl From<u16> for EndpointMultiplexingAddress {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<EndpointMultiplexingAddress> for u16 {
    #[inline]
    fn from(value: EndpointMultiplexingAddress) -> Self {
        value.0
    }
}

impl core::fmt::Debug for EndpointMultiplexingAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debugtuple = f.debug_struct("EndpointMultiplexingAddress");
        debugtuple.field(".0", &format_args!("{:#06x}", self.0));
        if let Some(description) = self.description() {
            debugtuple.field("description", &description);
        } else {
            // Elided for known values: Their range is probably clear from context.
            debugtuple.field(
                "range",
                if RANGE_COMPANY_SPECIFIC.contains(self) {
                    &"company specific"
                } else if RANGE_FREEUSE.contains(self) {
                    &"free use"
                } else if RANGE_PUBLIC_SPEC.contains(self) {
                    &"public specification"
                } else {
                    &"reserved"
                },
            );
        }
        debugtuple.finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for EndpointMultiplexingAddress {
    fn format(&self, f: defmt::Formatter<'_>) {
        if let Some(description) = self.description() {
            defmt::write!(
                f,
                "EndpointMultiplexingAddress {{ .0: {:#06x}, description: {} }}",
                self.0,
                description
            );
        } else {
            let range = if RANGE_COMPANY_SPECIFIC.contains(self) {
                &"company specific"
            } else if RANGE_FREEUSE.contains(self) {
                &"free use"
            } else if RANGE_PUBLIC_SPEC.contains(self) {
                &"public specification"
            } else {
                &"reserved"
            };
            defmt::write!(
                f,
                "EndpointMultiplexingAddress {{ .0: {:#06x}, range: {} }}",
                self.0,
                range
            );
        }
    }
}

// Those are not associated constants of EndpointMultiplexingAddress because struct associated
// constants can not be wildcard imported (and for those a wildcard import makes a lot of sense).

/// IPv6 Datagrams
///
/// See [IETF RFC8200: Internet Protocol, Version 6 (IPv6)](https://datatracker.ietf.org/doc/html/rfc8200)
pub const DATAGRAM_IPV6: EndpointMultiplexingAddress = EndpointMultiplexingAddress(0x8002);

/// IPv6 datagrams with header compression as defined in RFC6282
///
/// See [IETF RFC6282: Compression Format for IPv6 Datagrams over IEEE 802.15.4-Based Networks (IPv6 LoWPAN Header Compression)](https://datatracker.ietf.org/doc/html/rfc6282)
pub const DATAGRAM_6LO: EndpointMultiplexingAddress = EndpointMultiplexingAddress(0x8003);

/// Identifier for the Configuration Data Request
///
/// Configuration Data Request; TS.103.636-5 (rel. 2 or later), Annex C
pub const CONFIGURATION_DATA_REQUEST: EndpointMultiplexingAddress =
    EndpointMultiplexingAddress(0x8004);

/// Identifier for the Configuration Data Response
///
/// Configuration Data Response; TS.103.636-5 (rel. 2 or later), Annex C
pub const CONFIGURATION_DATA_RESPONSE: EndpointMultiplexingAddress =
    EndpointMultiplexingAddress(0x8005);

/// Free use range
///
/// Appendix A of ETSI TS 103 636-5 V2.1.1
pub const RANGE_FREEUSE: core::ops::RangeInclusive<EndpointMultiplexingAddress> =
    EndpointMultiplexingAddress(0x0100)..=EndpointMultiplexingAddress(0x40ff);

/// Public specification range
///
/// Appendix A of ETSI TS 103 636-5 V2.1.1
pub const RANGE_PUBLIC_SPEC: core::ops::RangeInclusive<EndpointMultiplexingAddress> =
    EndpointMultiplexingAddress(0x8000)..=EndpointMultiplexingAddress(0x84ff);

/// Company specific EP range
///
/// Appendix A of ETSI TS 103 636-5 V2.1.1
pub const RANGE_COMPANY_SPECIFIC: core::ops::RangeInclusive<EndpointMultiplexingAddress> =
    EndpointMultiplexingAddress(0xa000)..=EndpointMultiplexingAddress(0xbfff);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range() {
        assert!(RANGE_PUBLIC_SPEC.contains(&DATAGRAM_6LO));
    }

    #[test]
    fn test_conversion() {
        assert_eq!(u16::from(CONFIGURATION_DATA_REQUEST), 0x8004);
        assert_eq!(
            EndpointMultiplexingAddress::from(0x8005),
            CONFIGURATION_DATA_RESPONSE
        );
    }

    #[test]
    fn test_usability() {
        assert!(matches!(EndpointMultiplexingAddress(0x8002), DATAGRAM_IPV6));

        // This doesn't work -- let's see if we find ourselves regretting this and revert back to
        // coap-numbers' style, but I think it'll work for here, because this will not be used by
        // such a large variety of special-purpose applications.

        // assert!(matches!(0x8002, EndpointMultiplexingAddress::DATAGRAM_IPV6.0));
    }

    #[test]
    fn test_debug() {
        extern crate std;
        use std::format;

        assert_eq!(
            &format!("{:?}", CONFIGURATION_DATA_RESPONSE),
            &"EndpointMultiplexingAddress { .0: 0x8005, description: \"Configuration Data Response\" }"
        );

        assert_eq!(
            &format!("{:?}", EndpointMultiplexingAddress(0x0123)),
            &"EndpointMultiplexingAddress { .0: 0x0123, range: \"free use\" }"
        );
    }
}
