//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::{
    entry,
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        Watchdog,
    },
};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{pac, sio::Sio};
use embedded_hal::pwm::SetDutyCycle;

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
    // External high-speed crystal on the pico board is 12Mhz
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
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
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let led_pin = pins.gpio15.into_push_pull_output();

    // The minimum PWM value (i.e. LED brightness) we want
    const LOW: u16 = 0;

    // The maximum PWM value (i.e. LED brightness) we want
    const HIGH: u16 = 25000;

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM7
    let pwm = &mut pwm_slices.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    // Output channel B on PWM7 to the LED pin
    let channel = &mut pwm.channel_b;
    channel.output_to(led_pin);
    // Infinite loop, fading LED up and down
    loop {
        // Ramp brightness up
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        // Ramp brightness down
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        delay.delay_ms(500);
    }
}
