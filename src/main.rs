#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

mod json;
mod file;
mod func;


pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[post("/")]
fn index() -> String {
    func::func::get_data()
}

#[options("/")]
fn options() {}


fn main() {
    let config = rocket::Config::build(rocket::config::Environment::Production)
        .address("127.0.0.1")
        .port(45613)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![index, options])
        .attach(CORS)
        .launch();
}
