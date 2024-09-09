// Blinky with embassy

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut col3 = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let mut row3 = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);

    col3.set_low();
    loop {
        row3.set_high();
        Timer::after_millis(300).await;
        row3.set_low();
        Timer::after_millis(300).await;
    }
}
