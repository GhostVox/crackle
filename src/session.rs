use crate::DB;
use crate::config::Config;
use crate::constants::EXPECTED_FORMAT;
use crate::error::FatalError;
use crate::game_engine::GameEngine;
use crate::input::InputSource;
use crate::output::OutputSink;
use colored::Colorize;

use rand::Rng;
use uuid::Uuid;
// The session module is the orchestrator of each game, getting the input type, creating the game engine and managing the game state.
pub struct Session<'c, I: InputSource, O: OutputSink> {
    session_id: uuid::Uuid,
    date: chrono::DateTime<chrono::Utc>,
    game_engine: GameEngine,
    db: DB,
    input_source: I,
    output_sink: O,
    number_of_guesses: u8,
    config: &'c Config,
}

impl<'c, I: InputSource, O: OutputSink> Session<'c, I, O> {
    fn new(input: I, output: O, config: &'c Config, db: DB) -> Self {
        Session {
            session_id: Uuid::new_v4(),
            date: chrono::Utc::now(),
            game_engine: GameEngine::new(),
            db,
            input_source: input,
            output_sink: output,
            number_of_guesses: 0,
            config,
        }
    }

    //IMPORTANT: we need to make sure the main function handles the errors propagated from here

    /// Starts the game session, initializes the game engine with the starting word .
    pub fn initalize(&mut self) -> Result<(), FatalError> {
        let words = self
            .db
            .get_top_words(self.config.get_limit())
            .map_err(FatalError::DatabaseError)?;
        let rng = rand::thread_rng().gen_range(0..words.len());

        let starting_word = words[rng].clone().as_str();
        self.number_of_guesses += 1;
        self.game_engine.set_starting_word(starting_word);

        Ok(())
    }

    fn start(&self) -> Result<(), FatalError> {
        if self.input_source.is_interactive() {
            welcome();
        }
        self.output_sink
            .output_guess(self.game_engine.get_current_guess())?;
        loop {
            self.input_source.get_feedback(self.session_id);
            let out_of_guesses = self.check_number_of_guesses();
            if out_of_guesses {
                self.output_sink.out_of_guesses()?;
                Ok(())
            }
        }
    }

    fn check_number_of_guesses(&self) -> bool {
        self.number_of_guesses >= self.config.get_max_guesses()
    }
}

fn welcome() {
    println!("Welcome to Crackle!\r\n");
    println!("I will give you a word to try based on positional frequency");
    println!(
        "To achieve this, you will need to enter {}, {}, and N for gray",
        "G for green".green(),
        "Y for yellow".yellow()
    );
    println!("Example: {}", EXPECTED_FORMAT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome() {
        welcome();
    }
}
