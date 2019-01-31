#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate chrono;
extern crate actix_web;

mod models;

//use models::person;
use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: &HttpRequest) -> impl Responder {
    //Json(person::Person.get(1));
    "Service working"
}

fn persons(_req: &HttpRequest) -> impl Responder {
    "Call the person actor. TODO"
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/person", |r| r.f(persons))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}