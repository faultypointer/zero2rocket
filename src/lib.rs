use rocket::{http::Status, Build, Rocket};

#[macro_use]
extern crate rocket;

#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

pub fn run() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check])
}
