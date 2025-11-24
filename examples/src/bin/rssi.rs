#![no_main]
#![no_std]

use ariel_os::debug::{exit, log::info, ExitCode};
use ariel_os::time::Timer;

#[ariel_os::task(autostart)]
async fn main() {
    let mut dect = ariel_os::hal::modem::take_modem().await;

    for _ in 0..30 {
        info!("DECT time is {:?}", dect.time_get().await);

        info!("Scanning band 1");
        for carrier in 1657..=1677 {
            if let Ok(rssi) = dect.rssi(carrier).await {
                info!("RSSI for {} at {}: {:?}", carrier, rssi.0, rssi.1.data());
            }
        }
        info!("Scanning band 2");
        for carrier in 1680..=1700 {
            if let Ok(rssi) = dect.rssi(carrier).await {
                info!("RSSI for {} at {}: {:?}", carrier, rssi.0, rssi.1.data());
            }
        }
        info!("Scanning band 9");
        for carrier in 1703..=1711 {
            if let Ok(rssi) = dect.rssi(carrier).await {
                info!("RSSI for {} at {}: {:?}", carrier, rssi.0, rssi.1.data());
            }
        }
        // Not scanning band 22 yet: That is weirdly spanning others

        // Probably out of reach? 400kHz area -- funny, it initializes and gives COMPLETED but no frames.
        dect.rssi(1).await.map(|_| ()).unwrap_err();

        Timer::after_millis(500).await;
    }

    exit(ExitCode::SUCCESS);
}
