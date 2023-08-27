use cached::proc_macro::once;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use reqwest::Client;
use rocket::http::Status;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum Difficulty {
    EASY,
    MEDIUM,
    HARD,
}

const DICTION_URL: &str = "https://www.mit.edu/~ecprice/wordlist.10000";

pub async fn run(client: &Client, word: &String, difficult: &Difficulty) -> String {
    let response = get_dictionary(client, difficult).await;
    let dicts: Vec<String> = match response {
        Some(res) => res.unwrap(),
        None => panic!("Unable to retrive dictionary"),
    };

    let rng = rand::thread_rng().gen_range(0..dicts.len());

    let computer_guess: &str = &dicts[rng];
    let word_without_newline = word.trim();

    if computer_guess == word_without_newline {
        format!(
            "Horray! you guezzed the word {} correctly",
            word_without_newline.to_string()
        )
    } else {
        format!(
            "oops! you guessed wrongly\ncomputer guess: {}",
            computer_guess
        )
    }
}

#[once(option = true, sync_writes = true)]
async fn get_dictionary(
    client: &Client,
    difficult: &Difficulty,
) -> Option<Result<Vec<String>, Status>> {
    let response = fetch_dictionary(client).await.or(Err(Status::NoContent));

    // split response into a vector of string
    let mut words: Vec<String> = response
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // randomize words to make sure even after shorten the list we still cover most alphabetical words
    words.shuffle(&mut thread_rng());

    Some(Ok(match difficult {
        Difficulty::EASY => get_easy_words(&words),
        Difficulty::MEDIUM => get_medium_words(&words),
        Difficulty::HARD => get_hard_words(&words),
    }))
}

async fn fetch_dictionary(client: &Client) -> Result<String, reqwest::Error> {
    let response = client.get(DICTION_URL).send().await.expect("Error");
    response.text().await
}

// difficult level reduced to 1000 words
fn get_easy_words(words: &Vec<String>) -> Vec<String> {
    (&words[0..words.len() / 10]).to_vec()
}

// difficult level reduced to 2000 words
fn get_medium_words(words: &Vec<String>) -> Vec<String> {
    (&words[0..words.len() / 5]).to_vec()
}

// difficult level reduced to 5000 words
fn get_hard_words(words: &Vec<String>) -> Vec<String> {
    (&words[0..words.len() / 2]).to_vec()
}
