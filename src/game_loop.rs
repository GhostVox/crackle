use crate::{database, filter_logic, word_analyzer::WordAnalyzer};
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    io::Error,
};
use thiserror::Error;

const EXPECTED_FORMAT: &str = "gyngy";
const MAX_GUESSES: u8 = 5;
const WORD_LENGTH: usize = 5;

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
    #[error("No possible guesses")]
    NoPossibleGuesses,
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
            if self.number_of_guesses > MAX_GUESSES {
                println!("Damn we will get it next time.");
                self.store_game_results()?;

                break;
            }
            let mut valid_input = false;
            let mut user_input = String::new();
            loop {
                if valid_input {
                    break;
                }
                match self.get_user_input() {
                    Ok(input) => {
                        valid_input = true;
                        user_input = input;
                    }
                    Err(_) => continue,
                }
            }

            self.parse_user_input(user_input);
            if self.check_for_win() {
                self.store_game_results()?;
                break;
            }
            // take users input from last guess and calculate new guess
            if let Some(next_possible_guesses) = self.get_next_guess() {
                println!("Next possible guesses: {next_possible_guesses}");
                self.current_word = next_possible_guesses;
                self.number_of_guesses += 1;
            } else {
                return Err(GameError::NoPossibleGuesses);
            }
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
        let excluded_chars = process_input_characters(self, &input);
        for char in excluded_chars.iter() {
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
    pub fn store_game_results(&self) -> Result<(), GameError> {
        let game_results = GameResults {
            word: self.current_word.clone(),
            number_of_guesses: self.number_of_guesses,
            win: self.check_for_win(),
        };
        // Store game_results in database or file
        self.db
            .store_game_results(game_results)
            .map_err(GameError::DatabaseError)?;
        println!("Game results stored successfully!");
        println!("See you tomorrow!");
        Ok(())
    }

    pub fn get_next_guess(&self) -> Option<String> {
        let pattern: String = self.answer.iter().collect::<String>();
        let potential_words = self.db.filter_words(&pattern);
        match potential_words {
            Ok(words) => {
                if words.is_empty() {
                    println!("No matching words found!");
                    return None;
                }
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
                word_analyzer
                    .get_most_probable_word()
                    .map(|word| word.as_str())
            }

            Err(err) => {
                println!("Error filtering words: {err}");
                None
            }
        }
    }
}

/// Checks the user's input for validity, ensuring it is exactly 5 characters long and contains only 'g', 'y', or 'n'.
fn check_input(input: &str) -> Result<(), InputError> {
    if input.len() != WORD_LENGTH {
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

fn process_input_characters(game: &mut GameLoop, input: &str) -> HashSet<char> {
    let mut excluded_chars = HashSet::new();
    for (i, c) in input.chars().enumerate() {
        match c {
            'g' => {
                let c = game.current_word.chars().nth(i).unwrap();

                game.answer[i] = c;
            }
            'y' => {
                let c = game.current_word.chars().nth(i).unwrap();
                excluded_chars.remove(&c);
                game.yellow_positions.insert((c, i), true);
                game.yellow_characters.insert(c, true);
            }
            'n' => {
                let c = game.current_word.chars().nth(i).unwrap();
                excluded_chars.insert(c);
            }
            _ => unreachable!(),
        }
    }
    excluded_chars
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

    #[test]
    fn test_check_for_win() {
        let mut game = create_test_game("apple");
        game.answer = ['a', 'p', 'p', 'l', 'e'];

        assert!(game.check_for_win());
    }

    // Helper function to create a test GameLoop
    fn create_test_game(current_word: &str) -> GameLoop {
        GameLoop {
            number_of_guesses: 1,
            excluded_characters: HashMap::new(),
            yellow_positions: HashMap::new(),
            yellow_characters: HashMap::new(),
            current_word: current_word.to_string(),
            answer: ['_'; 5],
            db: database::DB::new_in_memory().unwrap(), // Assuming you have this constructor
        }
    }

    #[test]
    fn test_all_green_input() {
        let mut game = create_test_game("apple");
        let excluded_chars = process_input_characters(&mut game, "ggggg");

        // All characters should be in answer
        assert_eq!(game.answer, ['a', 'p', 'p', 'l', 'e']);

        // No excluded characters
        assert!(excluded_chars.is_empty());

        // No yellow positions or characters
        assert!(game.yellow_positions.is_empty());
        assert!(game.yellow_characters.is_empty());
    }

    #[test]
    fn test_all_gray_input() {
        let mut game = create_test_game("apple");
        let excluded_chars = process_input_characters(&mut game, "nnnnn");

        // Answer should remain unchanged
        assert_eq!(game.answer, ['_'; 5]);

        // All characters should be excluded
        let expected_excluded: HashSet<char> = ['a', 'p', 'l', 'e'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // No yellow positions or characters
        assert!(game.yellow_positions.is_empty());
        assert!(game.yellow_characters.is_empty());
    }

    #[test]
    fn test_all_yellow_input() {
        let mut game = create_test_game("apple");
        let excluded_chars = process_input_characters(&mut game, "yyyyy");

        // Answer should remain unchanged
        assert_eq!(game.answer, ['_'; 5]);

        // No excluded characters (yellow chars are removed from excluded set)
        assert!(excluded_chars.is_empty());

        // All positions should be in yellow_positions
        assert!(game.yellow_positions.contains_key(&('a', 0)));
        assert!(game.yellow_positions.contains_key(&('p', 1)));
        assert!(game.yellow_positions.contains_key(&('p', 2)));
        assert!(game.yellow_positions.contains_key(&('l', 3)));
        assert!(game.yellow_positions.contains_key(&('e', 4)));

        // All unique characters should be in yellow_characters
        assert!(game.yellow_characters.contains_key(&'a'));
        assert!(game.yellow_characters.contains_key(&'p'));
        assert!(game.yellow_characters.contains_key(&'l'));
        assert!(game.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_mixed_input() {
        let mut game = create_test_game("apple");
        let excluded_chars = process_input_characters(&mut game, "gyngy");

        // Check answer: positions 0 and 3 should be set
        assert_eq!(game.answer[0], 'a'); // green
        assert_eq!(game.answer[1], '_'); // yellow
        assert_eq!(game.answer[2], '_'); // gray
        assert_eq!(game.answer[3], 'l'); // green
        assert_eq!(game.answer[4], '_'); // yellow

        // Check excluded characters: only 'p' (position 2) should be excluded
        let expected_excluded: HashSet<char> = ['p'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check yellow positions
        assert!(game.yellow_positions.contains_key(&('p', 1))); // position 1 yellow
        assert!(game.yellow_positions.contains_key(&('e', 4))); // position 4 yellow

        // Check yellow characters
        assert!(game.yellow_characters.contains_key(&'p'));
        assert!(game.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_duplicate_characters_mixed() {
        let mut game = create_test_game("hello");
        let excluded_chars = process_input_characters(&mut game, "gnygy");

        // Check answer
        assert_eq!(game.answer[0], 'h'); // green
        assert_eq!(game.answer[1], '_'); // gray
        assert_eq!(game.answer[2], '_'); // yellow
        assert_eq!(game.answer[3], 'l'); // green
        assert_eq!(game.answer[4], '_'); // yellow

        // 'e' is gray at position 1, but yellow at position 2
        // So 'e' should NOT be in excluded_chars (yellow takes precedence)
        let expected_excluded: HashSet<char> = ['e'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Yellow positions should contain the yellow instances
        assert!(game.yellow_positions.contains_key(&('l', 2)));
        assert!(game.yellow_positions.contains_key(&('o', 4)));

        // Yellow characters should contain the yellow chars
        assert!(game.yellow_characters.contains_key(&'l'));
        assert!(game.yellow_characters.contains_key(&'o'));
    }

    #[test]
    fn test_same_char_gray_then_yellow() {
        let mut game = create_test_game("speed");
        let excluded_chars = process_input_characters(&mut game, "nygyn");

        // 's' is gray at position 0, 'e' is yellow at position 2
        // Since 'e' has yellow, it should be removed from excluded
        let expected_excluded: HashSet<char> = ['s', 'd'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check that 'e' is in yellow collections
        assert!(game.yellow_positions.contains_key(&('e', 3)));
        assert!(game.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_empty_excluded_when_all_chars_are_green_or_yellow() {
        let mut game = create_test_game("trust");
        let excluded_chars = process_input_characters(&mut game, "gygyg");

        // No characters should be excluded since all are either green or yellow
        assert!(excluded_chars.is_empty());

        // Check green positions
        assert_eq!(game.answer[0], 't');
        assert_eq!(game.answer[2], 'u');
        assert_eq!(game.answer[4], 't');

        // Check yellow positions
        assert!(game.yellow_positions.contains_key(&('r', 1)));
        assert!(game.yellow_positions.contains_key(&('s', 3)));
    }

    #[test]
    fn test_word_with_repeated_chars() {
        let mut game = create_test_game("paper");
        let excluded_chars = process_input_characters(&mut game, "gnyyn");

        // First 'p' is green, second 'p' is yellow
        assert_eq!(game.answer[0], 'p');
        assert_eq!(game.answer[1], '_');
        assert_eq!(game.answer[2], '_');
        assert_eq!(game.answer[3], '_');
        assert_eq!(game.answer[4], '_');

        // 'a' and 'r' should be excluded, but not 'p' or 'e'
        let expected_excluded: HashSet<char> = ['a', 'r'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check yellow positions and characters
        assert!(game.yellow_positions.contains_key(&('p', 2))); // second p is yellow
        assert!(game.yellow_positions.contains_key(&('e', 3))); // e is yellow
        assert!(game.yellow_characters.contains_key(&'p'));
        assert!(game.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_maintains_existing_game_state() {
        let mut game = create_test_game("world");

        // Set up some existing state
        game.answer[0] = 'w';
        game.yellow_positions.insert(('x', 0), true);
        game.yellow_characters.insert('x', true);

        let excluded_chars = process_input_characters(&mut game, "nggnn");

        // New state should be added
        assert_eq!(game.answer[0], 'w'); // should be overwritten
        assert_eq!(game.answer[1], 'o');
        assert_eq!(game.answer[2], 'r');

        // Existing yellow state should be preserved
        assert!(game.yellow_positions.contains_key(&('x', 0)));
        assert!(game.yellow_characters.contains_key(&'x'));

        // New excluded characters
        let expected_excluded: HashSet<char> = ['w', 'l', 'd'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);
    }
}
