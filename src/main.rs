use actix_web::{
    get, post, Error,
    web::{self, Json, Path},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    App, HttpResponse, HttpServer, Responder,
};
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;

use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;


#[derive(Serialize, Deserialize)]
struct LightToggleRequest {
    pin: u8,
    state: bool,
}

#[get("/test")]
async fn test() -> impl Responder {
    let _ = blink(21);
    let _ = blink(20);
    let _ = blink(16);
    let _ = blink(12);
    let _ = blink(25);
    let _ = blink(24);
    let _ = blink(23);
    let _ = blink(18);

    let _ = blink(21);
    let _ = blink(20);
    let _ = blink(16);
    let _ = blink(12);
    let _ = blink(25);
    let _ = blink(24);
    let _ = blink(23);
    let _ = blink(18);

    let _ = blink(21);
    let _ = blink(20);
    let _ = blink(16);
    let _ = blink(12);
    let _ = blink(25);
    let _ = blink(24);
    let _ = blink(23);
    let _ = blink(18);

    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/light")]
async fn light(req: Json<LightToggleRequest>) -> impl Responder {
    println!("blinking on {}", req.pin);
    let _ = blink(req.pin);
    HttpResponse::Ok().body("Hey there!")
}

#[post("/blink/on")]
async fn blinkon(req: Json<LightToggleRequest>) -> impl Responder {
    println!("blinking on {}", req.pin);
    let _ = blink_on(req.pin);
    HttpResponse::Ok().body("Hey there!")
}

#[post("/blink/off")]
async fn blinkoff(req: Json<LightToggleRequest>) -> impl Responder {
    println!("blinking on {}", req.pin);
    let _ = blink_off(req.pin);
    HttpResponse::Ok().body("Hey there!")
}

pub struct SayHi;
// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}
pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        // Claim the locks required

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            // release the locks
            println!("Hi from response");
            Ok(res)
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()           
            .wrap(SayHi)
            .service(test)
            .service(light)
            .service(blinkoff)
            .service(blinkon)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn blink(pin: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new().unwrap().get(pin).unwrap().into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(250));
    pin.set_low();

    Ok(())
}

fn blink_on(pin: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new().unwrap().get(pin).unwrap().into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(250));

    Ok(())
}

fn blink_off(pin: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new().unwrap().get(pin).unwrap().into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_low();
    thread::sleep(Duration::from_millis(250));

    Ok(())
}
