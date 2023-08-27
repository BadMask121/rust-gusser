#[get("/")]
pub fn render() -> &'static str {
    "Welcome home"
}
