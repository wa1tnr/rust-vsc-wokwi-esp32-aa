#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio5.into_push_pull_output();

    // led.set_high().unwrap();
    led.set_low().unwrap();

    let mut delay = Delay::new(&clocks);
    println!("Good on you in 2024 -- go for it!\r");
    loop {
        led.toggle().unwrap();
        delay.delay_ms(40u32);
        led.toggle().unwrap();
        delay.delay_ms(1400u32);
    }
}
// end.
