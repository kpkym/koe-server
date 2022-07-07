#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::Read;

use rocket::{Data, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::ext::IntoCollection;
use rocket::http::Header;
use serde_json::Value;

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

#[post("/", data = "<data>")]
fn index(data: Data) -> String {
    let mut req = String::new();

    data.open().read_to_string(&mut req).unwrap();

    let v: Value = json::json::get_type(&req);
    let key = v.as_object().unwrap().keys().last().unwrap();

    let code = json::json::list_code(v.get(key).unwrap());

    println!("{:?}", code);
    let mut result: String = String::new();
    match key.as_str() {
        "get_data" => {
            result = func::func::get_data(&code)
        }

        &_ => {}
    }

    result
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
