//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::{
    entry,
    hal::gpio::{bank0::Gpio15, FunctionSio, Pin, PullDown, SioOutput},
};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
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

    let led_pin = pins.gpio15.into_push_pull_output();
    blink_solution_with_for(led_pin);
}

fn blink_solution_with_for(mut led_pin: Pin<Gpio15, FunctionSio<SioOutput>, PullDown>) -> ! {
    const LED_BLINK_PERIOD: i32 = 10_000_000;
    loop {
        info!("on!");
        for _ in 0..LED_BLINK_PERIOD {
            led_pin.set_high().unwrap();
        }
        info!("off!");
        for _ in 0..LED_BLINK_PERIOD {
            led_pin.set_low().unwrap();
        }
    }
}
