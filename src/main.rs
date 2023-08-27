#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
mod services;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like something went wrong on our end"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
        )
        .register("/", catchers![internal_error])
        .mount("/", routes::mount())
}
