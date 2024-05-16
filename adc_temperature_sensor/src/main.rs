//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP15
//!
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::pac;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    info!("Program start: temperature sensor in free-running mode");
    let mut pac = pac::Peripherals::take().unwrap();

    // EXAMPLE05: Temperature sensor in free-running mode
    // https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/adc/index.html#free-running-mode-with-fifo
    use bsp::hal::adc::Adc;
    // Enable adc
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    // Enable the temperature sensor
    let mut temperature_sensor = adc.take_temp_sensor().unwrap();

    // Configure & start capturing to the fifo:
    let mut fifo = adc
        .build_fifo()
        .clock_divider(0, 0) // sample as fast as possible (500ksps. This is the default)
        .set_channel(&mut temperature_sensor)
        .start();

    loop {
        if fifo.len() > 0 {
            // Read one captured ADC sample from the FIFO:
            let temperature_adc_counts: u16 = fifo.read();
            info!(
                "adc value: {} -> temperature = {} C",
                temperature_adc_counts,
                convert_to_celsius(temperature_adc_counts)
            );
        }
    }
}

fn convert_to_celsius(raw_temp: u16) -> f32 {
    // According to chapter 4.9.5. Temperature Sensor in RP2040 datasheet
    let temp = 27.0 - (raw_temp as f32 * 3.3 / 4096.0 - 0.706) / 0.001721;
    let sign = if temp < 0.0 { -1.0 } else { 1.0 };
    let rounded_temp_x10: i16 = ((temp * 10.0) + 0.5 * sign) as i16;
    (rounded_temp_x10 as f32) / 10.0
}
