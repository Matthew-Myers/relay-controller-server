extern crate gpio;

use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main () {
    // Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();

    let mut value = false;

    thread::spawn(move || loop {
        gpio24.set_value(value).expect("could not set gpio24");
        println!("toggle");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });

    // The main thread will simply display the current value of GPIO23 every 100ms.
    loop {
        thread::sleep(time::Duration::from_millis(100));
    }
}