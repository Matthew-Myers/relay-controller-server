
use rouille::router;
use rouille::Request;
use rouille::Response;


use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main() {
    print!("STARTING SERVER ON PORT 80");
    let mut gpio21 = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();

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

fn note_routes(request: &Request) -> Response {
    router!(request,


        (GET) (/helloworld) => {
            // This route returns the list of notes. We perform the query and output it as JSON.

            let message = "hello world";


            Response::json(&message)
        },


        (GET) (/helloworld2) => {
            // This route returns the list of notes. We perform the query and output it as JSON.

            let message = "hello world 2";

            Response::json(&message)
        },



        // If none of the other blocks matches the request, return a 404 response.
        _ => Response::empty_404()
    )
}



/*
    // Let's open GPIO21 and -24, e.g. on a Raspberry Pi 2.

}*/
