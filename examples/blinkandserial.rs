#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial};

// use nb::block;

// blink and serial write example

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA1 as output.
    let mut led = gpioa.pa7.into_push_pull_output();

    // Choose TX / RX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    // Configure the serial peripheral.
    let serial = dp
        .USART2
        .usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut tx, _rx) = serial.split();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // core::fmt::Write is implemented for tx.
    //writeln!(tx, "Hello, world!").unwrap();

    loop {
        writeln!(tx, "Hello, world!").unwrap();
        
        led.set_high().unwrap();
        delay.delay_ms(1000_u16);

        led.set_low().unwrap();
        delay.delay_ms(1000_u16);
    }
}