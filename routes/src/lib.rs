#![forbid(unsafe_code)]

use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
