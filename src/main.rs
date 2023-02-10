mod word;
mod wordle;

use regex::Regex;
use std::io::{self, Write};

use word::Word;
use wordle::Wordle;

// Word list file path constant
pub const WORD_LIST_FILE: &str = "./words.txt";

// Util function that gets input from user
fn get_input(prompt_str: &str) -> Result<String, io::Error> {
    // Buffer that stores the user input
    let mut buffer = String::new();
    // Prompt user
    print!("{}", prompt_str);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

// Parses the input from user and returns a 'Word' instance
fn parse_wordle_input(str: &str) -> Option<Word> {
    // Regex expression
    const INPUT_REGEX: &str = r#"^([a-z]{5})\s([xyg]{5})$"#;
    // Regex instance
    let regex = Regex::new(INPUT_REGEX).unwrap();
    // If user input doesn't match input regex, return None
    if !regex.is_match(str) {
        return None;
    }
    // If user input matches the input regex, split the input by whitespace
    let mut split_parts = str.trim().split_ascii_whitespace();
    // Create and return a 'Word' instance from user input
    Some(Word::new(split_parts.next().unwrap(), split_parts.next().unwrap()))
}

fn start_program(wordle: &mut Wordle) {
    // Main program loop
    loop {
        // Get input from user
        let user_input = get_input("<< ");
        if user_input.is_err() {
            eprintln!("Couldn't get input!");
            continue;
        }
        let user_input = user_input.unwrap();
        // Parse user input into wordle word instance
        let user_word = parse_wordle_input(user_input.as_str());
        if user_word.is_none() {
            eprintln!(
                "Couldn't parse word. Please input word in the following format\n\t<WORD> <STATE>"
            );
            continue;
        }
        let user_word = user_word.unwrap();
        // Process the wordle word
        wordle.process(&user_word);
    }
}

fn main() {
    let mut wordle = Wordle::new();

    // start_program(&mut wordle);
    wordle.test();
}
