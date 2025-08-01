use crate::error::RecoverableError;
use crate::filter_logic;
use crate::word_analyzer::WordAnalyzer;
use std::collections::HashMap;
use std::collections::HashSet;
// the game engine, manages game state and logic for the game
#[derive(Debug)]
pub struct GameEngine {
    excluded_characters: HashMap<char, bool>,
    // uses a key of character + position
    yellow_positions: HashMap<(char, usize), bool>,
    // yellow characters that must be in the word somewhere.
    yellow_characters: HashMap<char, bool>,
    answer: [char; 5],
    current_guess: String,
}
impl Default for GameEngine {
    fn default() -> Self {
        Self {
            excluded_characters: HashMap::new(),
            yellow_positions: HashMap::new(),
            yellow_characters: HashMap::new(),
            answer: ['_'; 5],
            current_guess: String::new(),
        }
    }
}
#[allow(clippy::uninlined_format_args)]
impl std::fmt::Display for GameEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Just add '#' to the debug formatter
        write!(f, "{:#?}", self)
    }
}
impl GameEngine {
    /// Creates a new instance of the game engine.
    pub fn new() -> Self {
        Self {
            excluded_characters: HashMap::new(),
            yellow_positions: HashMap::new(),
            yellow_characters: HashMap::new(),
            answer: ['_'; 5],
            current_guess: String::new(),
        }
    }

    /// Sets the starting word for the engine to use for the first user input.
    pub fn set_starting_word(&mut self, starting_word: String) {
        self.current_guess = starting_word;
    }

    /// Gives the current guess of the engine back to the caller.
    pub fn get_current_guess(&self) -> &str {
        &self.current_guess
    }

    /// Parses the user input by getting a list of excluded characters from process_input_characters, It then checks the engine's state to make sure the characters in vector from process_input_characters are not included in the answer and updates the engine's state accordingly.
    pub fn parse_input(&mut self, input: &str) {
        let excluded_chars = self.process_input_characters(input);
        for char in excluded_chars.iter() {
            if self.answer.contains(char) {
                self.yellow_characters.insert(*char, true);
                continue;
            }
            self.excluded_characters.insert(*char, true);
        }
    }

    /// Processes the input characters by comparing against the current guess and updates the engine's state accordingly.
    fn process_input_characters(&mut self, input: &str) -> HashSet<char> {
        let mut excluded_chars = HashSet::new();
        for (i, c) in input.chars().enumerate() {
            match c {
                'g' => {
                    let c = self.current_guess.chars().nth(i).unwrap();

                    self.answer[i] = c;
                }
                'y' => {
                    let c = self.current_guess.chars().nth(i).unwrap();
                    excluded_chars.remove(&c);
                    self.yellow_positions.insert((c, i), true);
                    self.yellow_characters.insert(c, true);
                }
                'n' => {
                    let c = self.current_guess.chars().nth(i).unwrap();
                    if self.yellow_characters.contains_key(&c) || self.answer.contains(&c) {
                        self.yellow_positions.insert((c, i), true);
                        continue;
                    }
                    excluded_chars.insert(c);
                }
                _ => unreachable!(),
            }
        }
        excluded_chars
    }

    /// Takes a list of possible words for the next guess and calls filter_logic::filter_potential_words to get a list of words that match the current constraints of the game. Then it calculates the probabilities of the subset of words, and gets the most probable word.
    pub fn get_next_guess(
        &mut self,
        possible_words: Vec<String>,
    ) -> Result<String, RecoverableError> {
        let filtered_words = filter_logic::filter_potential_words(
            possible_words,
            &self.yellow_positions,
            &self.excluded_characters,
            &self.current_guess,
            &self.yellow_characters,
        );
        let mut word_analyzer = WordAnalyzer::new();
        for word in filtered_words {
            let _result = word_analyzer.analyze_word(&word);
        }
        word_analyzer.finalize_probabilities();
        let next_guess = word_analyzer
            .get_most_probable_word()
            .map(|word| word.as_str());
        match next_guess {
            Some(word) => {
                self.current_guess = word.to_string();
                Ok(word.to_string())
            }
            None => Err(RecoverableError::NoGuessFound),
        }
    }

    /// Returns the current state of the answer to the caller to be used to query the database.
    pub fn get_pattern(&self) -> String {
        let pattern: String = self.answer.iter().collect::<String>();
        pattern
    }
    /// Checks if the current guess matches the answer.
    pub fn check_for_win(&self) -> bool {
        !self.answer.contains(&'_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_engine(starting_word: &str) -> GameEngine {
        let mut engine = GameEngine::new();
        engine.set_starting_word(String::from(starting_word));
        engine
    }

    #[test]
    fn test_check_for_win() {
        let mut engine = create_test_engine("apple");
        engine.parse_input("ggggg");
        assert!(engine.check_for_win());
    }

    #[test]
    fn test_get_pattern() {
        let mut engine = create_test_engine("apple");
        engine.parse_input("gngng");
        assert_eq!(engine.get_pattern(), "a_p_e");
    }

    #[test]
    fn test_setting_starting_word() {
        let mut engine = create_test_engine("water");
        engine.process_input_characters("ggggg");
        assert_eq!(engine.answer, ['w', 'a', 't', 'e', 'r']);
    }

    #[test]
    fn test_all_green_input() {
        let mut engine = create_test_engine("apple");
        let excluded_chars = engine.process_input_characters("ggggg");

        // All characters should be in answer
        assert_eq!(engine.answer, ['a', 'p', 'p', 'l', 'e']);

        // No excluded characters
        assert!(excluded_chars.is_empty());

        // No yellow positions or characters
        assert!(engine.yellow_positions.is_empty());
        assert!(engine.yellow_characters.is_empty());
    }

    #[test]
    fn test_all_gray_input() {
        let mut engine = create_test_engine("apple");
        let excluded_chars = engine.process_input_characters("nnnnn");

        // Answer should remain unchanged
        assert_eq!(engine.answer, ['_'; 5]);

        // All characters should be excluded
        let expected_excluded: HashSet<char> = ['a', 'p', 'l', 'e'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // No yellow positions or characters
        assert!(engine.yellow_positions.is_empty());
        assert!(engine.yellow_characters.is_empty());
    }

    #[test]
    fn test_all_yellow_input() {
        let mut engine = create_test_engine("apple");
        let excluded_chars = engine.process_input_characters("yyyyy");

        // Answer should remain unchanged
        assert_eq!(engine.answer, ['_'; 5]);

        // No excluded characters (yellow chars are removed from excluded set)
        assert!(excluded_chars.is_empty());

        // All positions should be in yellow_positions
        assert!(engine.yellow_positions.contains_key(&('a', 0)));
        assert!(engine.yellow_positions.contains_key(&('p', 1)));
        assert!(engine.yellow_positions.contains_key(&('p', 2)));
        assert!(engine.yellow_positions.contains_key(&('l', 3)));
        assert!(engine.yellow_positions.contains_key(&('e', 4)));

        // All unique characters should be in yellow_characters
        assert!(engine.yellow_characters.contains_key(&'a'));
        assert!(engine.yellow_characters.contains_key(&'p'));
        assert!(engine.yellow_characters.contains_key(&'l'));
        assert!(engine.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_mixed_input() {
        let mut engine = create_test_engine("apple");
        let excluded_chars = engine.process_input_characters("gyngy");

        // Check answer: positions 0 and 3 should be set
        assert_eq!(engine.answer[0], 'a'); // green
        assert_eq!(engine.answer[1], '_'); // yellow
        assert_eq!(engine.answer[2], '_'); // gray
        assert_eq!(engine.answer[3], 'l'); // green
        assert_eq!(engine.answer[4], '_'); // yellow

        // Check excluded characters: nothing should be excluded because the answer contains p so we can't exclude it from the database query.
        let expected_excluded: HashSet<char> = [].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check yellow positions
        assert!(engine.yellow_positions.contains_key(&('p', 1))); // position 1 yellow
        assert!(engine.yellow_positions.contains_key(&('e', 4))); // position 4 yellow

        // Check yellow characters
        assert!(engine.yellow_characters.contains_key(&'p'));
        assert!(engine.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_duplicate_characters_mixed() {
        let mut engine = create_test_engine("hello");
        let excluded_chars = engine.process_input_characters("gnygy");

        // Check answer
        assert_eq!(engine.answer[0], 'h'); // green
        assert_eq!(engine.answer[1], '_'); // gray
        assert_eq!(engine.answer[2], '_'); // yellow
        assert_eq!(engine.answer[3], 'l'); // green
        assert_eq!(engine.answer[4], '_'); // yellow

        // 'e' is gray at position 1, but yellow at position 2
        // So 'e' should NOT be in excluded_chars (yellow takes precedence)
        let expected_excluded: HashSet<char> = ['e'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Yellow positions should contain the yellow instances
        assert!(engine.yellow_positions.contains_key(&('l', 2)));
        assert!(engine.yellow_positions.contains_key(&('o', 4)));

        // Yellow characters should contain the yellow chars
        assert!(engine.yellow_characters.contains_key(&'l'));
        assert!(engine.yellow_characters.contains_key(&'o'));
    }

    #[test]
    fn test_same_char_gray_then_yellow() {
        let mut engine = create_test_engine("speed");
        let excluded_chars = engine.process_input_characters("nygyn");

        // 's' is gray at position 0, 'e' is yellow at position 2
        // Since 'e' has yellow, it should be removed from excluded
        let expected_excluded: HashSet<char> = ['s', 'd'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check that 'e' is in yellow collections
        assert!(engine.yellow_positions.contains_key(&('e', 3)));
        assert!(engine.yellow_characters.contains_key(&'e'));
    }

    #[test]
    fn test_empty_excluded_when_all_chars_are_green_or_yellow() {
        let mut engine = create_test_engine("trust");

        let excluded_chars = engine.process_input_characters("gygyg");

        // No characters should be excluded since all are either green or yellow
        assert!(excluded_chars.is_empty());

        // Check green positions
        assert_eq!(engine.answer[0], 't');
        assert_eq!(engine.answer[2], 'u');
        assert_eq!(engine.answer[4], 't');

        // Check yellow positions
        assert!(engine.yellow_positions.contains_key(&('r', 1)));
        assert!(engine.yellow_positions.contains_key(&('s', 3)));
    }

    #[test]
    fn test_word_with_repeated_chars() {
        let mut engine = create_test_engine("paper");
        let excluded_chars = engine.process_input_characters("gnyyn");

        // First 'p' is green, second 'p' is yellow
        assert_eq!(engine.answer[0], 'p');
        assert_eq!(engine.answer[1], '_');
        assert_eq!(engine.answer[2], '_');
        assert_eq!(engine.answer[3], '_');
        assert_eq!(engine.answer[4], '_');

        // 'a' and 'r' should be excluded, but not 'p' or 'e'
        let expected_excluded: HashSet<char> = ['a', 'r'].iter().cloned().collect();
        assert_eq!(excluded_chars, expected_excluded);

        // Check yellow positions and characters
        assert!(engine.yellow_positions.contains_key(&('p', 2))); // second p is yellow
        assert!(engine.yellow_positions.contains_key(&('e', 3))); // e is yellow
        assert!(engine.yellow_characters.contains_key(&'p'));
        assert!(engine.yellow_characters.contains_key(&'e'));
    }
}
