#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::*;
use rocket::response::content::Json;

#[get("/status")]
fn status() -> Json<&'static str> {
    Json("{
        \"status\": \"success\",
        \"message\": \"Server running\"
    }")
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![status])
        .launch();
}
