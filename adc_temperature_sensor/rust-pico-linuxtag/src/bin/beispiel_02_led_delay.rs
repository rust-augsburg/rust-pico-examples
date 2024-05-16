//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15.
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry,
    gpio::{bank0::Gpio15, FunctionSio, Pin, PullDown, SioOutput},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let led_pin = pins.gpio15.into_push_pull_output();
    blink_solution_with_delay(led_pin, delay);
}

fn blink_solution_with_delay(
    mut led_pin: Pin<Gpio15, FunctionSio<SioOutput>, PullDown>,
    mut delay: Delay,
) -> ! {
    const LED_BLINK_PERIOD_MS: u32 = 500;
    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(LED_BLINK_PERIOD_MS);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(LED_BLINK_PERIOD_MS);
    }
}

// End of file
