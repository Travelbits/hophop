// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Minimal receive example
//!
//! While this is running, peers executing the nRF dect_shell example and running `dect ping -c`
//! produce visible traffic.
#![no_std]
#![no_main]

use ariel_os::debug::log::{info, warn, Hex};

use ts_103_636_numbers as numbers;
use ts_103_636_utils as utils;

fn log_header(header: &[u8]) {
    // Following ETSI TS 103 636-4 V2.1.1 Section 6.2
    let hdr_format = header[0] >> 5;
    let packet_len = header[0] & 0x0f;
    let packet_len_units = if header[0] & 0x10 == 0 {
        "subslots"
    } else {
        "slots"
    };
    let short_nid = header[1];
    let transmitter_id = u16::from_be_bytes(header[2..4].try_into().unwrap());
    let transmit_power = header[4] >> 4;
    // Weird enough, in the 5-byte header the MSB of this is reserved, but we can still
    // decode it this way:
    let df_mcs = header[4] & 0x0f;

    match header.len() {
        5 => {
            info!(
                "Header details: format {} length {} {}, nid {}, from {}, tx power {}, df_mcs {}",
                hdr_format,
                packet_len,
                packet_len_units,
                short_nid,
                transmitter_id,
                transmit_power,
                df_mcs
            );
        }
        10 => {
            let receiver_id = u16::from_be_bytes(header[5..7].try_into().unwrap());
            // Ignoring remaining feedback info for the moment; its interpretation depends on hdr_format
            // (although that really only tells if that's reserved or used).
            info!(
                "Header details: format {} length {} {}, nid {}, from {} to {}, tx power {}, df_mcs {}",
                hdr_format,
                packet_len,
                packet_len_units,
                short_nid,
                transmitter_id,
                receiver_id,
                transmit_power,
                df_mcs
            );
        }
        _ => unreachable!("Header length is always 5 or 10"),
    }
}

fn log_data(data: &[u8]) {
    // Following ETSI TS 103 636-4 V2.1.1 Section 6.3
    let version = data[0] >> 6;
    if version == 3 {
        warn!("Can not decode dect_shell ping (or whatever nonstandard version this is)");
        return;
    }
    if version != numbers::mac_pdu::VERSION {
        warn!("Unknown MAC version.");
        return;
    }
    let mac_sec_version = data[0] >> 6;
    let mac_hdr_type = data[0] & 0x0f;
    let mac_hdr_type_name = match mac_hdr_type {
        // FIXME: Add Formatter?
        numbers::mac_pdu::header_type::DATA_MAC_PDU => "DATA MAC PDU",
        numbers::mac_pdu::header_type::BEACON => "Beacon",
        numbers::mac_pdu::header_type::UNICAST => "Unicast",
        numbers::mac_pdu::header_type::RD_BROADCAST => "RD Broadcast",
        _ => "unknown",
    };
    info!(
        "Header data: MAC security {}, header type {} {}",
        mac_sec_version, mac_hdr_type, mac_hdr_type_name
    );
    let end_common_header = match mac_hdr_type {
        numbers::mac_pdu::header_type::DATA_MAC_PDU => {
            let reset = (data[1] & 0x10) >> 4;
            let seqno = (data[1] as u16 & 0x0f) << 8 | (data[2] as u16);

            let transmitter = &data[4..8];
            info!("DATA MAC PDU details: reset {}, seqno {}, from {}", reset, seqno, Hex(&transmitter));
            3
        }
        numbers::mac_pdu::header_type::BEACON => {
            let long_nid = &data[1..4];
            let transmitter = &data[4..8];
            info!(
                "Beacon details: Network {}, transmitter {}",
                Hex(&long_nid), Hex(&transmitter),
            );
            8
        }
        numbers::mac_pdu::header_type::UNICAST => {
            let reset = (data[1] & 0x10) >> 4;
            let mac_sequence = data[1] & 0x0f;
            let seqno = data[2];
            let receiver = &data[3..7];
            let transmitter = &data[7..11];
            info!(
                "Unicast details: reset {}, mac_sequence {}, seqno {}, to {} from {}",
                reset, mac_sequence, seqno, Hex(&receiver), Hex(&transmitter),
            );
            11
        }
        numbers::mac_pdu::header_type::RD_BROADCAST => {
            let reset = (data[1] & 0x10) >> 4;
            let seqno = (data[1] as u16 & 0x0f) << 8 | (data[2] as u16);
            let transmitter = &data[3..7];
            info!(
                "RD Broadcast details: reset {}, seqno {}, from {}",
                reset, seqno, Hex(&transmitter),
            );
            7
        }
        _ => {
            info!("Unknown common header, can not decode further");
            return;
        }
    };
    if mac_sec_version != 0 {
        info!("No link-layer security implemented, bailing.");
        return;
    }
    for item in utils::mac_ie::InformationElement::parse_stream(&data[end_common_header..]) {
        let Ok(item) = item else {
            warn!("Failed to parse item, aborting.");
            return;
        };
        info!("IE {:?}, payload {:?}", item.ie_number(), item.payload())
    }
    info!("Complete message processed.");
}

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = ariel_os::hal::modem::take_modem().await;

    for _ in 0..300 {
        if let Some(received) = dect
            .rx()
            .await
            .expect("Receive operation failed as a whole")
        {
            let start = received.pcc_time();
            let pcc = received.pcc();
            let pdc = received.pdc();
            if let (Ok(start), Ok(pcc), Ok(pdc)) = (start, pcc, pdc) {
                info!("Received at {}: {:?} {:?}", start, pcc, pdc);
                log_header(pcc);
                log_data(pdc);
            } else {
                warn!(
                    "Received partial transmission: {:?} {:?} {:?}",
                    start, pcc, pdc
                );
            }
        }
    }

    panic!("If we want to be able to re-flash, we better things at some point to avoid going through unlock again.");
}
