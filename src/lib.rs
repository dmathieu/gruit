#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::from_env()).mount("/", routes![index])
}
