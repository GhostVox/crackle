use crate::DB;
use crate::config::Config;
use crate::error::FatalError;
use crate::game_engine::GameEngine;
use crate::input::InputSource;
use crate::output::OutputSink;
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
    number_of_guesses: usize,
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

    /// Starts the game session, initializes the game engine with the starting word .
    pub fn initalize(&mut self) -> Result<(), FatalError> {
        let words = self
            .db
            .get_top_words(self.config.get_limit())
            .map_err(FatalError::DatabaseError)?;
        let rng = rand::thread_rng().gen_range(0..words.len());

        let starting_word = words[rng].clone().as_str();
        self.number_of_guesses += 1;
        self.output_sink.welcome(&starting_word)?;
        self.game_engine.set_starting_word(starting_word);

        Ok(())
    }
}
