use crate::config::Config;
use crate::constants::EXPECTED_FORMAT;
use crate::error::FatalError;
use crate::game_engine::GameEngine;
use crate::input::InputSource;
use crate::output::OutputSink;
use crate::{DB, logs};
use colored::Colorize;
use std::fmt::Display;

use rand::Rng;
use uuid::Uuid;
#[derive(Debug)]
pub enum SessionType {
    Interactive,
    Test,
    Api,
}

impl SessionType {
    /// Converts the enum to a string slice for storage or display.
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionType::Interactive => "Interactive",
            SessionType::Test => "Test",
            SessionType::Api => "Api",
        }
    }
}

impl Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct SessionResults {
    pub session_id: Uuid,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub session_type: String,
    pub word: String,
    pub number_of_guesses: u8,
    pub win: bool,
}

// The session module is the orchestrator of each game, getting the input type, creating the game engine and managing the game state.
pub struct Session<'c, 'a, I: InputSource, O: OutputSink> {
    session_id: uuid::Uuid,
    session_type: SessionType,
    start_date: chrono::DateTime<chrono::Utc>,
    game_engine: GameEngine,
    result_db: &'a DB,
    in_memory_db: &'a DB,
    input_source: I,
    output_sink: O,
    words_guessed: Vec<String>,
    number_of_guesses: u8,
    config: &'c Config,
}
impl<'c, 'a, I: InputSource, O: OutputSink> Display for Session<'c, 'a, I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Session ID: {:?}\n
            Session Type: {:?}\n
            Start Date: {:?}\n
            Words Guessed: {:#?}\n
            Game Engine: {}\n",
            self.session_id,
            self.session_type,
            self.start_date,
            self.words_guessed,
            self.game_engine
        )
    }
}

impl<'c, 'a, I: InputSource, O: OutputSink> Session<'c, 'a, I, O> {
    pub fn new(
        session_type: SessionType,
        input: I,
        output: O,
        config: &'c Config,
        result_db: &'a DB,
        in_memory_db: &'a DB,
    ) -> Self {
        Session {
            session_id: Uuid::new_v4(),
            session_type,
            start_date: chrono::Utc::now(),
            game_engine: GameEngine::new(),
            result_db,
            in_memory_db,
            input_source: input,
            output_sink: output,
            words_guessed: Vec::new(),
            number_of_guesses: 0,
            config,
        }
    }

    //IMPORTANT: we need to make sure the main function handles the errors propagated from here

    /// Starts the game session, initializes the game engine with the starting word .
    pub fn initialize(&mut self) -> Result<(), FatalError> {
        let words = self
            .in_memory_db
            .get_top_words(self.config.get_limit())
            .map_err(FatalError::DatabaseError)?;
        let rng = rand::thread_rng().gen_range(0..words.len());

        let starting_word = words[rng].clone().as_str();
        self.words_guessed.push(starting_word.clone());
        self.game_engine.set_starting_word(starting_word);

        Ok(())
    }

    pub fn start_interactive(&mut self) -> Result<(), FatalError> {
        // welcome the user
        welcome();

        // output the starting guess
        self.output_sink
            .output_guess(self.game_engine.get_current_guess())?;

        loop {
            // Get user feedback on the last guess
            let user_input = match self.input_source.get_feedback() {
                Ok(input) => input,
                Err(e) => return Err(FatalError::IOError(e)),
            };
            self.number_of_guesses += 1;
            // parse the user input
            self.game_engine.parse_input(&user_input);
            // check if the game is won
            if self.game_engine.check_for_win() {
                return self.store_session_results();
            }
            // check if the game is lost
            let out_of_guesses = self.out_of_guesses();
            if out_of_guesses {
                println!("Out of guesses!");
                return self.store_session_results();
            }
            let pattern = self.game_engine.get_pattern();
            let possible_words = self.in_memory_db.filter_words(&pattern)?;

            let next_guess = match self.game_engine.get_next_guess(possible_words) {
                Ok(guess) => guess,
                Err(e) => {
                    println!("I am stumped! {e}");
                    let session_state = format!("{self}");
                    logs::log_session_state(session_state)?;

                    return self.store_session_results();
                }
            };

            self.words_guessed.push(next_guess.clone());
            self.output_sink.output_guess(&next_guess)?;
        }
    }

    pub fn store_session_results(&self) -> Result<(), FatalError> {
        let session_results = self.get_session_results();
        // Store game_results in database or file
        self.result_db
            .store_session_results(&session_results)
            .map_err(FatalError::DatabaseError)?;
        println!("Game results stored successfully!");
        println!("See you tomorrow!");
        Ok(())
    }

    fn out_of_guesses(&self) -> bool {
        self.number_of_guesses >= self.config.get_max_guesses()
    }

    fn get_session_results(&self) -> SessionResults {
        SessionResults {
            session_id: self.session_id,
            start_date: self.start_date,
            end_date: chrono::Utc::now(),
            session_type: self.session_type.as_str().to_string(),
            word: self.game_engine.get_pattern(),
            number_of_guesses: self.number_of_guesses,
            win: self.game_engine.check_for_win(),
        }
    }
}

fn welcome() {
    println!("Welcome to Crackle!\r\n");
    println!("I will give you a word to try based on positional frequency");
    println!(
        "To achieve this, you will need to enter {}, {}, and {}",
        "G for green".green(),
        "Y for yellow".yellow(),
        "N for gray".bright_black()
    );
    println!("Example: {EXPECTED_FORMAT}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome() {
        welcome();
    }
}
