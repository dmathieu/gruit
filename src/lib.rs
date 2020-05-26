#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod config;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::from_env()).mount("/", routes![routes::index])
}
