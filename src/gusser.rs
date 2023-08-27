use rand::prelude::*;
use std::io;

fn get_dictionary() -> Vec<&'static str> {
    let words = vec![
        "hello", "nice", "come", "go", "love", "hate", "some", "stuff", "will", "use",
    ];

    return words;
}

fn guess(word: String) {
    let dicts: Vec<&str> = get_dictionary();
    let rng = rand::thread_rng().gen_range(0..dicts.len());

    let computer_guess: &str = dicts[rng];

    let word_without_newline = word.trim(); // Trim the newline character

    if computer_guess == word_without_newline {
        println!(
            "Hurray!! you guessed the word {} correctly",
            word_without_newline
        );
    } else {
        println!(
            "oops! you guessed wrongly\ncomputer guess: {}",
            computer_guess
        );
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter your guess:\n");

    let user_input = io::stdin().read_line(&mut input);

    match user_input {
        Ok(_) => guess(input),
        Err(e) => println!("Error occured {}", e),
    }
}
