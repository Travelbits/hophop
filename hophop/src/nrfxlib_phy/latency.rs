// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use nrf_modem::nrfxlib_sys::*;

/// Data is stored in a macro because there is no `PartialEq` on it, but we still want to do a
/// comparison, and matches! works on the literal form the macro gives. This is a purely
/// module-local implementation choice.
macro_rules! latency_info {
    () => {
        nrf_modem_dect_phy_latency_info {
            radio_mode: [
                nrf_modem_dect_phy_latency_info__bindgen_ty_1 {
                    scheduled_operation_transition: 25920,
                    scheduled_operation_startup: 0,
                    radio_mode_transition: [6912, 6912, 34905],
                },
                nrf_modem_dect_phy_latency_info__bindgen_ty_1 {
                    scheduled_operation_transition: 25920,
                    scheduled_operation_startup: 87782,
                    radio_mode_transition: [45273, 6912, 21427],
                },
                nrf_modem_dect_phy_latency_info__bindgen_ty_1 {
                    scheduled_operation_transition: 26956,
                    scheduled_operation_startup: 42854,
                    radio_mode_transition: [45273, 41472, 21427],
                },
            ],
            operation: nrf_modem_dect_phy_latency_info__bindgen_ty_2 {
                receive: nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_1 {
                    idle_to_active: 22118,
                    active_to_idle_rssi: 13132,
                    active_to_idle_rx: 12441,
                    active_to_idle_rx_rssi: 16588,
                    stop_to_rf_off: 14169,
                },
                transmit: nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_2 {
                    idle_to_active: 29030,
                    active_to_idle: 7603,
                },
            },
            stack: nrf_modem_dect_phy_latency_info__bindgen_ty_3 {
                initialization: 2764800,
                deinitialization: 62208,
                configuration: 7119360,
                activation: 2972160,
                deactivation: 58752,
            },
        }
    };
}

/// Latency as reported by nRF nr+ firmware 1.1.0.
///
/// Until we find that we actually need all of this, checking at startup allows us to reach into
/// the const for access whenever we need it, without needing to worry about how to get data
/// around.
#[expect(dead_code, reason = "users are TBD")]
pub const LATENCY_INFO: nrf_modem_dect_phy_latency_info = latency_info!();

/// Checks for identity between the `latency_info` argument and the [`LATENCY_INFO`] constant.
///
/// This is a dedicated function because we are not in control of the underlying type and can not
/// easily derive `PartialEq` for it.
pub fn latency_is_expected(latency_info: &nrf_modem_dect_phy_latency_info) -> bool {
    matches!(*latency_info, latency_info!())
}
