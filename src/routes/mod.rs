use rocket::Route;
pub mod app;
pub fn mount() -> Vec<Route> {
    routes![app::render]
}
