use std::collections::HashMap;
// the game engine, manages game state and logic for the game

pub struct GameEngine {
    number_of_guesses: u8,
    excluded_characters: HashMap<char, bool>,
    // uses a key of character + position
    yellow_positions: HashMap<(char, usize), bool>,
    // yellow characters that must be in the word somewhere.
    yellow_characters: HashMap<char, bool>,
    answer: [char; 5],
    current_guess: String,
}

impl GameEngine {
    /// Creates a new instance of the game engine.
    pub fn new() -> Self {
        Self {
            number_of_guesses: 0,
            excluded_characters: HashMap::new(),
            yellow_positions: HashMap::new(),
            yellow_characters: HashMap::new(),
            answer: ['_'; 5],
            current_guess: String::new(),
        }
    }

    pub fn set_starting_word(&mut self, starting_word: String) {
        self.current_guess = starting_word;
    }
}
