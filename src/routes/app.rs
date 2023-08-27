use reqwest::Client;
use rocket::State;

use crate::services;
use std::str::FromStr;

#[post("/<difficult>/<word>")]
pub async fn render(difficult: &str, word: &str, client: &State<Client>) -> String {
    services::guezzer::run(
        client,
        &word.to_string(),
        &services::guezzer::Difficulty::from_str(&difficult.to_uppercase()).unwrap(),
    )
    .await
}
