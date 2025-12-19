// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::MutexGuard};
use nrf_modem::{ErrorSource, nrfxlib_sys};

use super::{DECT_EVENTS, DectEvent, DectPhy, MixedError, RECVBUF};

/// Resulting data slice of a single RSSI measurement.
///
/// This keeps a lock on the receive buffer, and must therefore be dropped before the next attempt
/// to perform any other operation.
pub struct RssiResult<'a>(
    MutexGuard<'static, CriticalSectionRawMutex, heapless::Vec<u8, 2400>>,
    core::ops::Range<usize>,
    // This ensures that a result is used before the next attempt to receive something (as
    // that would panic around locking RECV_BUF).
    core::marker::PhantomData<&'a mut ()>,
);

impl RssiResult<'_> {
    pub fn data(&self) -> &[u8] {
        &self.0[self.1.clone()]
    }
}

impl DectPhy {
    pub async fn rssi(&mut self, carrier: u16) -> Result<(u64, RssiResult<'_>), MixedError> {
        self.clear_recvbuf();

        // Relevant DECT constant timing parameters are 1 frame = 10ms, each 10ms frame is composed
        // of 24 slots,

        // - Reporting interval is every 12 or 24 slots. This is consistent with the delta of
        //   starting times being precisely 691200 (24 slots = 10ms, on a 69.120MHz clock), or
        //   345600 (12 slots = 5ms).
        //
        // - Depending on the reporting interval there are 240 or 120 values, so single reading
        //   takes 2880 clock ticks, or 10 readings per slot, which corresponds to lowest number of
        //   ODFM symbols (for µ=1).
        //
        // - Requesting a duration of N gives 5*N readings. This is given in subslots, which for
        //   µ=1 is 2 subslots per slot, and thus matches 10 readings per slot, 5 per subslot.

        let params = nrfxlib_sys::nrf_modem_dect_phy_rssi_params {
            start_time: 0,
            handle: 1234567,
            carrier,
            duration: 48, // in subslots; 1 full report
            reporting_interval: nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_24_SLOTS, // 24 slots = 10ms
        };
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_rssi(&raw const params) }.into_result()?;

        let mut result = None;

        loop {
            match DECT_EVENTS.receive().await.event {
                DectEvent::Rssi(start, range) => {
                    debug_assert!(result.is_none(), "Sequence violation");
                    result = Some((
                        start,
                        range.expect("We requested just one run, that fits in the receive buffer"),
                    ));
                }
                DectEvent::Completed(Ok(())) => {
                    break;
                }
                DectEvent::Completed(e) => e?,
                _ => panic!("Sequence violation"),
            }
        }

        let Some(result) = result else {
            // FIXME: Verify that it's an actual completion error that happens when requesting an
            // unsupported channel.
            panic!("Sequence violation");
        };

        Ok((
            result.0,
            RssiResult(
                RECVBUF
                    .try_lock()
                    .expect("Was checked before, and ISR users release this before returning"),
                result.1,
                core::marker::PhantomData,
            ),
        ))
    }
}
