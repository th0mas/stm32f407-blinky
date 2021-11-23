//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpiod = dp.GPIOD.split();
        let mut orange = gpiod.pd13.into_push_pull_output();
        let mut red = gpiod.pd14.into_push_pull_output();
        let mut blue = gpiod.pd15.into_push_pull_output();
        let mut green = gpiod.pd12.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

        loop {
            // On for 1s, off for 1s.
            red.set_high();
            orange.set_low();
            blue.set_high();
            green.set_low();
            delay.delay_ms(1000_u32);
            red.set_low();
            orange.set_high();
            blue.set_low();
            green.set_high();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}