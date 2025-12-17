//! Ping/pong example
//!
//! See <https://github.com/ariel-os/hophop/issues/12> for initial documentation.
#![no_std]
#![no_main]

use ariel_os::debug::log::{Hex, info, warn};
use ariel_os_boards::pins;

use ts_103_636_numbers as numbers;
use ts_103_636_utils as utils;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::ButtonPeripherals) {
    let mut dect = hophop_examples::dect::DectPhy::init_inside_ariel().await.unwrap();

    let button0 = ariel_os::gpio::Input::new(peripherals.button0, ariel_os::gpio::Pull::Up);

    loop {
        // FIXME: This is an exactly 1-second receive; that'll need API changes, but for this very
        // moment we can just leave it at that.
        let received = dect
            .rx()
            .await
            .expect("Receive operation failed as a whole");

        if let Some(received) = received {
            let start = received.pcc_time();
            let pcc = received.pcc();
            let pdc = received.pdc();
            if let (Ok(start), Ok(pcc), Ok(pdc)) = (start, pcc, pdc) {
                info!("Received at {}: {:?} {:?}", start, pcc, pdc);
            } else {
                warn!(
                    "Received partial transmission: {:?} {:?} {:?}",
                    start, pcc, pdc
                );
            }
        }

        if button0.is_low() {
            let pcc = &[
                // header format 000, 1 subslots
                0x01,
                // short networkID
                0x41,
                // Transmitter identity
                0x12,
                0x34,
                // Transmit power and DF MCS as in what we've seen from dect_shell beacons
                0x70,
            ];
            let mut pdc_buf = heapless::Vec::<u8, 256>::new();
            pdc_buf
                .extend_from_slice(&[
                    // version 0, no security; beacon.
                    0x01,
                    // beacon details:
                    // full network ID
                    0x41, 0x41, 0x41,
                    // full sender ID
                    0xfe, 0xdc, 0x12, 0x34
                ])
                .unwrap();

            // FIXME CONTINUE HERE: We don't have a time we *can* include, because right now
            // `.tx()` sends ASAP and doesn't even tell *when* it sent.

            // For the time being we accept that the PCC (which is copy-pasted from beacon
            // messages) has a 50 byte payload, and we pad accordingly:
            //
            // ... or we just send in a subslot and then it's 17 length (1 subslot) or 33 (2
            // subslots)
            const LEN: usize = 17;

            assert!(pdc_buf.len() < LEN);
            while pdc_buf.len() < LEN {
                // Single paddings are easier than calculating back whether the rest fits in a u8
                // or u16 padding; on the long run, we'll have a "pad it up" function, or just emit
                // an indefinite-length padding, as that (juding from Wireshark) auto-fills all the
                // rest.
                utils::mac_ie::InformationElement::new_5bit(
                    numbers::mac_ie::ie5bit_len0::PADDING,
                    &[],
                )
                .unwrap()
                .serialize(&mut pdc_buf)
                .unwrap();
            }

            dect.tx(pcc, &pdc_buf).await.unwrap();

            info!("Sent {} bytes PDC data", pdc_buf.len());
        }
    }
}
