#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod beer;
mod config;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::from_env()).mount("/", routes![routes::index, routes::post_beer])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_rocket() {
        let rocket = rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/");
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
