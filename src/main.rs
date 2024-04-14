
use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main() {
    print!("STARTING SERVER ON PORT 80");
    let mut gpio21 = gpio::sysfs::SysFsGpioOutput::open(22).unwrap();

    let mut value = false;
            
    thread::spawn(move || loop {
        gpio21.set_value(value).expect("could not set gpio24");
        println!("toggle");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });

    // The main thread will simply display the current value of GPIO23 every 100ms.
    loop {
        thread::sleep(time::Duration::from_millis(100));
    }
}
