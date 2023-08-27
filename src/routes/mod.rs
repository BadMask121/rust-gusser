use rocket::Route;
pub mod home;
pub fn mount() -> Vec<Route> {
    routes![home::render]
}
