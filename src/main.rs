#[macro_use]
extern crate rocket;

mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes::mount())
}
