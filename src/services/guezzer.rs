use rand::prelude::*;

fn get_dictionary() -> Vec<&'static str> {
    let words = vec![
        "hello", "nice", "come", "go", "love", "hate", "some", "stuff", "will", "use",
    ];

    return words;
}

fn guess(word: String) -> String {
    let dicts: Vec<&str> = get_dictionary();
    let rng = rand::thread_rng().gen_range(0..dicts.len());

    let computer_guess: &str = dicts[rng];

    let word_without_newline = word.trim(); // Trim the newline character

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

pub fn run(input: &String) -> String {
    guess(input.to_string())
}
