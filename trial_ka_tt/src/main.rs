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

    // 0   2   4   5  12  13  14  16  17  18  19  21 22 23  25   26 
    // let mut led0  = io.pins.gpio0.into_push_pull_output();
    let mut leds = [
      io.pins.gpio1.into_push_pull_output().degrade(),
      io.pins.gpio10.into_push_pull_output().degrade(),
      io.pins.gpio19.into_push_pull_output().degrade(),
      io.pins.gpio18.into_push_pull_output().degrade(),
      io.pins.gpio4.into_push_pull_output().degrade(),
      io.pins.gpio5.into_push_pull_output().degrade(),
      io.pins.gpio6.into_push_pull_output().degrade(),
      io.pins.gpio7.into_push_pull_output().degrade(),
      io.pins.gpio8.into_push_pull_output().degrade(),
      io.pins.gpio9.into_push_pull_output().degrade(),
    ];

    // let mut led1  = io.pins.gpio1.into_push_pull_output();
    // let mut led2  = io.pins.gpio2.into_push_pull_output();
    // let mut led3  = io.pins.gpio3.into_push_pull_output();
    // let mut led4  = io.pins.gpio4.into_push_pull_output();
    // let mut led5  = io.pins.gpio5.into_push_pull_output();
    // let mut led6  = io.pins.gpio6.into_push_pull_output();
    // let mut led7  = io.pins.gpio7.into_push_pull_output();
    // let mut led8  = io.pins.gpio8.into_push_pull_output();
    // let mut led9  = io.pins.gpio9.into_push_pull_output();
    // let mut led10  = io.pins.gpio10.into_push_pull_output();
    // let mut led11  = io.pins.gpio11.into_push_pull_output();
    // let mut led12 = io.pins.gpio12.into_push_pull_output();
    // let mut led13 = io.pins.gpio13.into_push_pull_output();
    // let mut led14 = io.pins.gpio14.into_push_pull_output();
    // let mut led15  = io.pins.gpio15.into_push_pull_output();
    // let mut led16 = io.pins.gpio16.into_push_pull_output();
    // let mut led17 = io.pins.gpio17.into_push_pull_output();

    // let mut led18 = io.pins.gpio18.into_push_pull_output();
    // let mut led19 = io.pins.gpio19.into_push_pull_output();
    // let mut led20 = io.pins.gpio20.into_push_pull_output();
    // let mut led21 = io.pins.gpio21.into_push_pull_output();
    // let mut led22 = io.pins.gpio22.into_push_pull_output();
    // let mut led23 = io.pins.gpio23.into_push_pull_output();
    // let mut led24 = io.pins.gpio24.into_push_pull_output();
    // let mut led25 = io.pins.gpio25.into_push_pull_output();
    // let mut led26 = io.pins.gpio26.into_push_pull_output();
    // let mut led27 = io.pins.gpio27.into_push_pull_output();
    // let mut led28 = io.pins.gpio28.into_push_pull_output();
    // let mut led29 = io.pins.gpio29.into_push_pull_output();



    // led.set_high().unwrap();
    // led0.set_low().unwrap();
    led1.set_low().unwrap();
    // led2.set_low().unwrap();
    led3.set_low().unwrap();
    // led4.set_low().unwrap();
    // led5.set_low().unwrap();
    led6.set_low().unwrap();
    led7.set_low().unwrap();
    led8.set_low().unwrap();
    led9.set_low().unwrap();
    led10.set_low().unwrap();
    led11.set_low().unwrap();
    // led12.set_low().unwrap();
    // led13.set_low().unwrap();
    // led14.set_low().unwrap();
    led15.set_low().unwrap();
    // led16.set_low().unwrap();
    // led17.set_low().unwrap();
    // led18.set_low().unwrap();
    // led19.set_low().unwrap();
    led20.set_low().unwrap();
    // led21.set_low().unwrap();
    // led22.set_low().unwrap();
    // led23.set_low().unwrap();
    led24.set_low().unwrap();
    // led25.set_low().unwrap();
    // led26.set_low().unwrap();
    led27.set_low().unwrap();
    led28.set_low().unwrap();
    led29.set_low().unwrap();

    let mut delay = Delay::new(&clocks);
    println!("CARL rusuf/rust-vsc--/src/main.rs\r");
    loop {

    // 1 3 6-11 15 20 24 27-29

        // led0.toggle().unwrap();
        led1.toggle().unwrap();
        // led2.toggle().unwrap();
        led3.toggle().unwrap();
        // led4.toggle().unwrap();
        // led5.toggle().unwrap();
        led6.toggle().unwrap();
        led7.toggle().unwrap();
        led8.toggle().unwrap();
        led9.toggle().unwrap();
        led10.toggle().unwrap();
        led11.toggle().unwrap();
        // led12.toggle().unwrap();
        // led13.toggle().unwrap();
        // led14.toggle().unwrap();
        // led16.toggle().unwrap();
        // led17.toggle().unwrap();
        // led18.toggle().unwrap();
        // led19.toggle().unwrap();



        // led21.toggle().unwrap();
        // led22.toggle().unwrap();
        // led23.toggle().unwrap();
        // led25.toggle().unwrap();
        // led26.toggle().unwrap();

        delay.delay_ms(40u32);

        led0.toggle().unwrap();
        led2.toggle().unwrap();
        led4.toggle().unwrap();
        led5.toggle().unwrap();
        led12.toggle().unwrap();
        led13.toggle().unwrap();
        led14.toggle().unwrap();
        led16.toggle().unwrap();
        led17.toggle().unwrap();
        led18.toggle().unwrap();
        led19.toggle().unwrap();
        led21.toggle().unwrap();
        led22.toggle().unwrap();
        led23.toggle().unwrap();
        led25.toggle().unwrap();
        led26.toggle().unwrap();

        delay.delay_ms(1400u32);
    }
}
// end.
