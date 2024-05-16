//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::{InputPin, OutputPin};
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{pac, sio::Sio};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio15.into_push_pull_output();
    let mut button_pin = pins.gpio14.into_pull_down_input();

    loop {
        if button_pin.is_high().unwrap() {
            // Button is pressed (active low)
            info!("button on!");
            led_pin.set_high().unwrap();
        } else {
            info!("button off!");
            led_pin.set_low().unwrap();
        }
    }
}
