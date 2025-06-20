use crate::{database, word_analyzer::Character};
use rand::Rng;
use rusqlite::Connection;

const EXPECTED_FORMAT: &str = "gyngy";
pub struct GameLoop {
    pub number_of_guesses: u8,
    pub excluded_characters: Vec<Character>,
    pub included_characters: Vec<Character>,
    pub current_word: String,
}

pub enum GameError {
    DatabaseError(rusqlite::Error),
    WordNotFoundError,
}
enum Answers {
    Green,
    Yellow,
    Gray,
}
impl GameLoop {
    pub fn new() -> Self {
        Self {
            number_of_guesses: 0,
            excluded_characters: Vec::new(),
            included_characters: Vec::new(),
            current_word: String::from("_____"),
        }
    }

    pub fn start(&mut self, db: database::DB) -> Result<(), GameError> {
        // Get the limit from the environment variable or default to 10
        let limit = std::env::var("LIMIT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
        // Get the top words from the database
        let top_words = db.get_top_words(limit);
        match top_words {
            Ok(words) => {
                let rng = rand::thread_rng().gen_range(0..limit);

                self.current_word = words[rng].clone().as_str();
            }
            Err(e) => return Err(GameError::DatabaseError(e)),
        }
        println!("Welcome to Crackle!");
        println!("I will give you a word to try based on positional frequency");
        println!(
            "All you will have to do is tell me which characters were in the right position so we can narrow down the possibilities"
        );
        println!(
            "To achieve this, you will need to enter G for green, Y for yellow, and N for gray"
        );

        println!("Starting game with word: {}", self.word);

        println!("Please enter which characters were in the right position");

        println!("Example: {}", EXPECTED_FORMAT);
        return Ok(());
    }

    pub fn get_user_input(&mut self) -> String {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!(
                "Sorry I couldn't read your input error:{e}, please try again. Expected format: {}",
                EXPECTED_FORMAT
            );
        }
        let input = input.trim().to_lowercase();
        if input == "exit" {
            println!("Exiting game");
            std::process::exit(0);
        }
        if input.len() != 5 {
            println!(
                "Sorry your input was not the correct length, please try again. Expected format: {}",
                EXPECTED_FORMAT
            );
        }
        if !input.chars().all(|c| c == 'g' || c == 'y' || c == 'n') {
            println!(
                "Sorry your input was not the correct format, please try again. Expected format: {}",
                EXPECTED_FORMAT
            );
        }
        input
    }
}
