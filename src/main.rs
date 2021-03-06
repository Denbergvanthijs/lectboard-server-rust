#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use image::ImageFormat;
use local_ipaddress;
use rocket::{
    data::{FromDataSimple, Outcome},
    http::Status,
    Data,
    Outcome::{Failure, Success},
    Request,
};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use serde::Serialize;
use std::{collections::HashMap, io::Read};

#[derive(Serialize)]
struct Host {
    hostname: String,
    port: u32,
}

#[derive(Debug)]
struct StoreImage {
    succes: bool,
}

impl FromDataSimple for StoreImage {
    // Based on https://stackoverflow.com/q/63301943
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> Outcome<Self, String> {
        let mut image = Vec::new();

        if let Err(e) = data.open().read_to_end(&mut image) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        match image::load_from_memory_with_format(&image, ImageFormat::Png) {
            Ok(_img) => {
                std::fs::write("static/result.png", &image).unwrap();
                Success(StoreImage { succes: true })
            }
            Err(e) => Failure((Status::NotFound, e.to_string())),
        }
    }
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[post("/image", data = "<_input>")]
fn post_image(_input: StoreImage) {}

#[get("/hostname", format = "json")]
fn get_hostname() -> Json<Host> {
    Json(Host {
        hostname: local_ipaddress::get().unwrap(),
        port: 5000,
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, post_image, get_hostname])
        .attach(Template::fairing())
}

fn main() {
    // use rocket::http::hyper::header::{CacheControl, CacheDirective, Headers};
    // let mut headers = Headers::new();
    // headers.set(CacheControl(vec![
    //     CacheDirective::NoCache,
    //     CacheDirective::NoStore,
    //     CacheDirective::MustRevalidate,
    //     CacheDirective::MaxAge(0),
    // ]));

    rocket().launch();
}
