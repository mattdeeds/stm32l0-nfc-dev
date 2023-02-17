#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};
use tmp1x2::{Tmp1x2, SlaveAddr};
use embedded_time::rate::*;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA1 as output for led
    let mut led = gpioa.pa7.into_push_pull_output();

    // configure I2C pins
    let sda = gpioa.pa10.into_open_drain_output();
    let scl = gpioa.pa9.into_open_drain_output();

    let i2c = dp.I2C1.i2c(sda, scl, Hertz(100_000), &mut rcc);
    let address = SlaveAddr::default();
    let mut sensor = Tmp1x2::new(i2c, address);

    loop {
        
        let temperature = sensor.read_temperature().unwrap();
        let delay_time;

        // Set delay time based on temperature
        // if temperature > 22.0 then fast blink else slow blink
        // temp should be over 22.0 in house

        if temperature > 22.0 {
            delay_time = 4000000;
        } else {
            delay_time = 20000000;
        }

        led.set_high().unwrap();
        delay(delay_time);

        led.set_low().unwrap();
        delay(delay_time);
    }
}