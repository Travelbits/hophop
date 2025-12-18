//! Processing between a MAC PDU and its IEs (Information Elements)

use super::ParsingError;

use ts_103_636_numbers as numbers;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MacHeaderType(u8);

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DataMacPdu<'buf>(&'buf [u8; 2]);
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Beacon<'buf>(&'buf [u8; 7]);
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Unicast<'buf>(&'buf [u8; 10]);
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RdBroadcast<'buf>(&'buf [u8; 6]);

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacCommonHeader<'buf> {
    DataMacPdu(DataMacPdu<'buf>),
    Beacon(Beacon<'buf>),
    Unicast(Unicast<'buf>),
    RdBroadcast(RdBroadcast<'buf>),
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header<'buf> {
    pub head: MacHeaderType,
    pub common: MacCommonHeader<'buf>,
    /// IE items; best used with the [`Self::tail_items()`] iterator
    pub tail: &'buf [u8],
}

impl<'buf> Header<'buf> {
    pub fn parse(mut buffer: &'buf [u8]) -> Result<Self, ParsingError> {
        let head = *buffer.split_off_first().ok_or(ParsingError)?;
        if head & 0xc0 != 0 {
            // version could have different structure
            return Err(ParsingError);
        }

        let common = match head & 0x0f {
            numbers::mac_pdu::header_type::DATA_MAC_PDU => MacCommonHeader::DataMacPdu(DataMacPdu(
                buffer
                    .split_off(..2)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::BEACON => MacCommonHeader::Beacon(Beacon(
                buffer
                    .split_off(..7)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::UNICAST => MacCommonHeader::Unicast(Unicast(
                buffer
                    .split_off(..10)
                    .ok_or(ParsingError)?
                    .try_into()
                    .expect("split_off length is guaranteed"),
            )),
            numbers::mac_pdu::header_type::RD_BROADCAST => {
                MacCommonHeader::RdBroadcast(RdBroadcast(
                    buffer
                        .split_off(..6)
                        .ok_or(ParsingError)?
                        .try_into()
                        .expect("split_off length is guaranteed"),
                ))
            }
            _ => return Err(ParsingError),
        };

        Ok(Self {
            head: MacHeaderType(head),
            common,
            tail: buffer,
        })
    }

    /// Convenience accessor for [`crate::mac_ie::InformationElement::parse_stream(Self::tail)`].
    pub fn tail_items(&self) -> impl Iterator<Item = Result<crate::mac_ie::InformationElement<'_>, ParsingError>> {
        crate::mac_ie::InformationElement::parse_stream(self.tail)
    }
}
