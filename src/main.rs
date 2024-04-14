use std::error::Error;
use std::thread;
use std::time::Duration;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let _ = blink();
    format!("Hello {name}!");
}

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 21;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn blink() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}

