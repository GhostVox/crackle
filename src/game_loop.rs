use crate::{database, word_analyzer::WordAnalyzer};
use rand::Rng;
use thiserror::Error;

const EXPECTED_FORMAT: &str = "gyngy";

pub struct GameLoop {
    pub number_of_guesses: u8,
    pub excluded_characters: Vec<char>,
    pub yellow_positions: Vec<(char, usize)>,
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
pub enum GameError {
    #[error("Database error: {0}")]
    DatabaseError(rusqlite::Error),
    #[error("Word not found error")]
    WordNotFoundError,
    #[error("Invalid input error")]
    InvalidInputError,
}

impl GameLoop {
    pub fn new(db: database::DB) -> Self {
        Self {
            number_of_guesses: 0,
            excluded_characters: Vec::new(),
            yellow_positions: Vec::new(),
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
            }
            Err(e) => return Err(GameError::DatabaseError(e)),
        }
        // Welcome the user
        self.welcome_msg();

        loop {
            if self.number_of_guesses > 6 {
                println!("Damn we will get it next time.");
                if let Err(e) = self.store_game_results() {
                    println!("Error storing game results: {}", e);
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
                    println!("Error storing game results: {}", e);
                    return Err(GameError::DatabaseError(e));
                }
                break;
            }
            // take users input from last guess and calculate new guess
            let next_possible_guesses = self.get_next_guess();
            println!("Next possible guesses: {:?}", next_possible_guesses);
            self.current_word = next_possible_guesses;

            self.number_of_guesses += 1;
        }

        return Ok(());
    }

    // Get user input
    pub fn get_user_input(&mut self) -> Result<String, GameError> {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!(
                "Sorry I couldn't read your input error:{e}, please try again. Expected format: {}",
                EXPECTED_FORMAT
            );
            return Err(GameError::InvalidInputError);
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
            return Err(GameError::InvalidInputError);
        }
        if !input.chars().all(|c| c == 'g' || c == 'y' || c == 'n') {
            println!(
                "Sorry your input was not the correct format, please try again. Expected format: {}",
                EXPECTED_FORMAT
            );
            return Err(GameError::InvalidInputError);
        }
        Ok(input)
    }

    // Parse user input and update game state
    pub fn parse_user_input(&mut self, input: String) {
        // if the guess has two of the same character's in the word and the first one in no we end up pushing that character into the excluded characters vector which causes any characters in the right spot to be excluded as well
        let mut temp: [(char, bool); 5] = [('_', false); 5];
        for (i, c) in input.chars().enumerate() {
            match c {
                'g' => {
                    let character = self.current_word.chars().nth(i).unwrap();
                    temp[i] = (character, false);
                    self.answer[i] = character;
                }
                'y' => {
                    let character = self.current_word.chars().nth(i).unwrap();
                    temp[i] = (character, false);
                    self.yellow_positions.push((character, i));
                }
                'n' => {
                    let character = self.current_word.chars().nth(i).unwrap();
                    temp[i] = (character, true);
                }
                _ => unreachable!(),
            }
        }
        for (character, excluded) in temp.iter() {
            if *excluded && !temp.iter().any(|(c, e)| c == character && !*e) {
                if !self.excluded_characters.contains(character) {
                    self.excluded_characters.push(*character);
                }
            }
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

        Ok(())
    }

    pub fn welcome_msg(&self) {
        println!("Welcome to Crackle!");
        println!("I will give you a word to try based on positional frequency");
        println!(
            "All you will have to do is tell me which characters were in the right position so we can narrow down the possibilities"
        );
        println!(
            "To achieve this, you will need to enter G for green, Y for yellow, and N for gray"
        );

        println!("Starting game with word: {}", self.current_word);

        println!("Please enter which characters were in the right position");

        println!("Example: {}", EXPECTED_FORMAT);
    }

    pub fn get_next_guess(&self) -> String {
        let pattern: String = self.answer.iter().collect::<String>();
        let potential_words = self.db.filter_words(&pattern);
        match potential_words {
            Ok(mut words) => {
                if words.is_empty() {
                    println!("No matching words found!");
                    String::new()
                } else {
                    words.retain(|word| {
                        !self
                            .excluded_characters
                            .iter()
                            .any(|&excluded_character| word.contains(excluded_character))
                    });
                    // Filter for yellow letters (must contain but not in wrong position)
                    words.retain(|word| {
                        self.yellow_positions.iter().all(|(ch, wrong_pos)| {
                            word.contains(*ch) && word.chars().nth(*wrong_pos) != Some(*ch)
                        })
                    });
                    let mut word_analyzer = WordAnalyzer::new();
                    for word in words {
                        let _result = word_analyzer.analyze_word(&word);
                    }
                    word_analyzer.finalize_probabilities();
                    if let Some(mpw) = word_analyzer.get_most_probable_word() {
                        String::from(mpw.as_str())
                    } else {
                        println!("No word found");
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                println!("Error filtering words: {}", err);
                String::new()
            }
        }
    }
}
