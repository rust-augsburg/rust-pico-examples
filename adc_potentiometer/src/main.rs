//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    info!("Program start: temperature sensor in free-running mode");
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // EXAMPLE06: ADC sensor in free-running mode
    // https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo
    use bsp::hal::adc::Adc;
    // Enable adc
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    // Configure GPIO26 as an ADC input
    let mut adc_pin_0 = bsp::hal::adc::AdcPin::new(pins.gpio26).unwrap();

    // Configure & start capturing to the fifo:
    let mut fifo = adc
        .build_fifo()
        .clock_divider(0, 0) // sample as fast as possible (500ksps. This is the default)
        .set_channel(&mut adc_pin_0)
        .start();

    loop {
        if fifo.len() > 0 {
            // Read one captured ADC sample from the FIFO:
            let adc_counts: u16 = fifo.read();
            info!("adc0 value: {}", adc_counts);
        }
    }
}
