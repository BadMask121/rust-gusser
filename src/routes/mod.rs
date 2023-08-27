use rocket::Route;
pub mod home;
pub mod user;

pub fn mount() -> Vec<Route> {
    routes![user::render, home::render]
}
