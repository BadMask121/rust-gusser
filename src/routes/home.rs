use crate::services;

#[post("/<word>")]
pub fn render(word: &str) -> String {
    let result = services::guezzer::run(&word.to_string());
    format!("{}", result)
}
