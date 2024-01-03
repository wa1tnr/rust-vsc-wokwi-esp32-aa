#![no_std]
#![no_main]

use esp_backtrace as _;

use esp_println::println;
use esp_println::print; // no newline!

use hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new(&clocks);

    delay.delay_ms(800u32);
    println!("\r\n  ticonda ro- ga - kit-KURGAN rusuf/main.rs\r");
    delay.delay_ms(1200u32);






    let mut leds = [

      // cannot locate onboard LED at all!
      io.pins.gpio4.into_push_pull_output().degrade(),
      io.pins.gpio5.into_push_pull_output().degrade(),
      io.pins.gpio12.into_push_pull_output().degrade(),
      io.pins.gpio15.into_push_pull_output().degrade(),
      io.pins.gpio17.into_push_pull_output().degrade(),
      io.pins.gpio18.into_push_pull_output().degrade(),
      io.pins.gpio19.into_push_pull_output().degrade(),
      io.pins.gpio21.into_push_pull_output().degrade()
    ];

    for led in &mut leds {
        led.set_low().unwrap();
    }






    let mut counted: i16 = 0;
    print!("\n");

    loop {
      counted = counted + 1;
      delay.delay_ms(3u32); // don't want a real delay
      let mut counter: i16 = 3;
      for led in &mut leds {
        counter = counter + 1;
        let _eselectedled: i16 = 16;
        if counter == 4 {
        let mut input_string: &str = " hi\n\n\n";
        input_string = input_string.trim();
        print!("\r  ");
        print!("{}",input_string);
        print!("t 4 ");
        // println!("{}", input_string.trim());
        // print!("{}", input_string.trim());
        }

        if counter == 5 {
          print!("  hit 5 ");
          counter = 11; // skip 6-11
        }

        if counter == 12 {
          print!("  hit 12 ");
          counter = 14; // skip 13-14
        }

        if counter == 15 {
          print!("  hit 15 ");
          counter = 16; // skip 16
        }

        if counter == 17 {
          print!("  hit 17 ");
        }

        if counter == 18 {
          print!("  hit 18 ");
        }

        if counter == 19 {
          print!("  hit 19 ");
          counter = 20; // skip 20
        }

        if counter == 21 {
          print!("  hit 21 ");
          print!("  ");
          print!("{}", counted); // moveable feast - last item
          print!(" ");
        }

        led.toggle().unwrap();
        delay.delay_ms(80u32);
        led.toggle().unwrap();
        delay.delay_ms(1400u32);
      }

      // -----------------------------------------------------------------------------------
      // -----------------------------------------------------------------------------------
      // string below this line is way way way long! -->
      // -----------------------------------------------------------------------------------
      print!("\r                                                                  RET\r");
      // -----------------------------------------------------------------------------------
      // -----------------------------------------------------------------------------------
    }
}
// end.
