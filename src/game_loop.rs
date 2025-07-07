use crate::{database, filter_logic, word_analyzer::WordAnalyzer};
use rand::Rng;
use std::{collections::HashMap, io::Error};
use thiserror::Error;

const EXPECTED_FORMAT: &str = "gyngy";

pub struct GameLoop {
    pub number_of_guesses: u8,
    pub excluded_characters: HashMap<char, bool>,
    // uses a key of character + position
    pub yellow_positions: HashMap<(char, usize), bool>,
    // yellow characters that must be in the word somewhere.
    pub yellow_characters: HashMap<char, bool>,

    pub current_word: String,
    pub answer: [char; 5],
    pub db: database::DB,
}

pub struct GameResults {
    pub word: String,
    pub number_of_guesses: u8,
    pub win: bool,
}

#[derive(Debug, Error)]
enum InputError {
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid length")]
    InvalidLength,
    #[error("Error parsing input")]
    ParseInput(Error),
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Database error: {0}")]
    DatabaseError(rusqlite::Error),
    #[error("Word not found error")]
    WordNotFoundError,
}

impl GameLoop {
    pub fn new(db: database::DB) -> Self {
        Self {
            number_of_guesses: 0,
            excluded_characters: HashMap::new(),
            yellow_positions: HashMap::new(),
            yellow_characters: HashMap::new(),

            current_word: String::from("_____"),
            answer: ['_'; 5], // this gets filled with characters that are green
            db,
        }
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        // Get the limit from the environment variable or default to 10
        let limit = std::env::var("LIMIT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
        // Get the top words from the database
        let top_words = self.db.get_top_words(limit);
        match top_words {
            Ok(words) => {
                let rng = rand::thread_rng().gen_range(0..limit);

                self.current_word = words[rng].clone().as_str();
                self.number_of_guesses = 1;
            }
            Err(e) => return Err(GameError::DatabaseError(e)),
        }
        // Welcome the user
        welcome_msg(&self.current_word);

        loop {
            if self.check_for_win() {
                println!("Congratulations! You guessed the word.");
                if let Err(e) = self.store_game_results() {
                    println!("Error storing game results: {e}");
                    return Err(GameError::DatabaseError(e));
                }
                break;
            }
            if self.number_of_guesses > 5 {
                println!("Damn we will get it next time.");
                if let Err(e) = self.store_game_results() {
                    println!("Error storing game results: {e}",);
                    return Err(GameError::DatabaseError(e));
                }
                break;
            }
            let mut valid_input = false;
            let mut user_input = String::new();
            loop {
                if valid_input {
                    break;
                }
                let input = self.get_user_input();
                match input {
                    Ok(input) => {
                        valid_input = true;
                        user_input = input;
                    }
                    Err(_) => continue,
                }
            }

            self.parse_user_input(user_input);
            if self.check_for_win() {
                if let Err(e) = self.store_game_results() {
                    println!("Error storing game results: {e}");
                    return Err(GameError::DatabaseError(e));
                }
                break;
            }
            // take users input from last guess and calculate new guess
            let next_possible_guesses = self.get_next_guess();
            println!("Next possible guesses: {next_possible_guesses}");
            self.current_word = next_possible_guesses;

            self.number_of_guesses += 1;
        }

        Ok(())
    }

    // Get user input
    fn get_user_input(&mut self) -> Result<String, InputError> {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!(
                "Sorry I couldn't read your input error:{e}, please try again. Expected format: {EXPECTED_FORMAT}"
            );
            return Err(InputError::ParseInput(e));
        }
        let input = input.trim().to_lowercase();
        if input == "exit" {
            println!("Exiting game");
            std::process::exit(0);
        }
        let input_ok = check_input(&input);
        match input_ok {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(input)
    }

    // Parse user input and update game state
    pub fn parse_user_input(&mut self, input: String) {
        // temp holds yellow characters and excluded characters
        let mut excluded_chars: HashMap<char, bool> = HashMap::new();
        for (i, c) in input.chars().enumerate() {
            match c {
                'g' => {
                    let c = self.current_word.chars().nth(i).unwrap();

                    self.answer[i] = c;
                }
                'y' => {
                    let c = self.current_word.chars().nth(i).unwrap();
                    excluded_chars.remove(&c);
                    self.yellow_positions.insert((c, i), true);
                    self.yellow_characters.insert(c, true);
                }
                'n' => {
                    let c = self.current_word.chars().nth(i).unwrap();
                    excluded_chars.insert(c, true);
                }
                _ => unreachable!(),
            }
        }
        for (char, _) in excluded_chars.iter() {
            // If the character was grey in that position, but was in the word earlier add it to the yellow positions stuct so we filter words with that character out later, but don't exclude words entirely with that character
            if self.answer.contains(char) {
                self.yellow_characters.insert(*char, true);
                continue;
            }
            self.excluded_characters.insert(*char, true);
        }
    }

    // compares answer with current_word if they match exactly we have won!
    pub fn check_for_win(&self) -> bool {
        !self.answer.contains(&'_')
    }

    // Clean up function Stores game results in database
    pub fn store_game_results(&self) -> Result<(), rusqlite::Error> {
        let game_results = GameResults {
            word: self.current_word.clone(),
            number_of_guesses: self.number_of_guesses,
            win: self.check_for_win(),
        };
        // Store game_results in database or file
        //
        self.db.store_game_results(game_results)?;
        println!("Game results stored successfully!");
        println!("See you tomorrow!");
        Ok(())
    }

    pub fn get_next_guess(&self) -> String {
        let pattern: String = self.answer.iter().collect::<String>();
        let potential_words = self.db.filter_words(&pattern);
        match potential_words {
            Ok(words) => {
                if words.is_empty() {
                    println!("No matching words found!");
                    String::new()
                } else {
                    let filtered_words = filter_logic::filter_potential_words(
                        words,
                        &self.yellow_positions,
                        &self.excluded_characters,
                        &self.current_word,
                        &self.yellow_characters,
                    );
                    let mut word_analyzer = WordAnalyzer::new();
                    for word in filtered_words {
                        let _result = word_analyzer.analyze_word(&word);
                    }
                    word_analyzer.finalize_probabilities();
                    if let Some(mpw) = word_analyzer.get_most_probable_word() {
                        mpw.as_str()
                    } else {
                        println!("No word found");
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                println!("Error filtering words: {err}");
                String::new()
            }
        }
    }
}

/// Checks the user's input for validity, ensuring it is exactly 5 characters long and contains only 'g', 'y', or 'n'.
fn check_input(input: &str) -> Result<(), InputError> {
    if input.len() != 5 {
        println!(
            "Sorry your input was not the correct length, please try again. Expected format: {EXPECTED_FORMAT}"
        );
        return Err(InputError::InvalidLength);
    }
    if !input.chars().all(|c| c == 'g' || c == 'y' || c == 'n') {
        println!(
            "Sorry your input was not the correct format, please try again. Expected format: {EXPECTED_FORMAT}"
        );
        return Err(InputError::InvalidFormat);
    }
    Ok(())
}

/// Welcome message for the game, takes the first word to start with as a parameter
pub fn welcome_msg(current_word: &str) -> String {
    let msg = format!("
Welcome to Crackle!\r\n
I will give you a word to try based on positional frequency\r\n
All you will have to do is tell me which characters were in the right position so we can narrow down the possibilities\r\n
To achieve this, you will need to enter G for green, Y for yellow, and N for gray\r\n
Starting game with word: {current_word}\r\n
Please enter which characters were in the right position\r\n
Example: {EXPECTED_FORMAT}\r\n");
    print!("{msg}");
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_input() {
        // Valid input: correct length and format
        assert!(check_input("gyngy").is_ok());
        // Invalid input: too long
        assert!(check_input("gyngyy").is_err());
        // Invalid input: too short
        assert!(check_input("gyn").is_err());
        // Invalid input: wrong characters
        assert!(check_input("abcde").is_err());
    }

    #[test]
    fn test_welcome_msg() {
        let msg = welcome_msg("apple");
        assert!(msg.contains("apple"));
        assert!(msg.contains(EXPECTED_FORMAT))
    }
}
