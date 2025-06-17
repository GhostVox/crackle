use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Character {
    character: u8,
    position: u8,
    probability: Option<u32>,
    frequency: u32,
}

/// Struct representing a character in a word, with methods for creating new instances, incrementing frequency, getting character, updating probability.
impl Character {
    fn new(character: u8, position: u8, probability: Option<u32>, frequency: u32) -> Self {
        Character {
            character,
            position,
            probability,
            frequency,
        }
    }

    fn increment_frequency(&mut self) {
        self.frequency += 1;
    }

    fn get_char(&self) -> char {
        self.character as char
    }

    fn update_probability(&mut self, total_frequency: u32) {
        self.probability = Some((self.frequency * 100) / total_frequency);
    }
}

// Word Struct
#[derive(Debug, Clone)]
pub struct Word {
    pub frequency: u32,
    pub total_probability: f64,
    pub word: [Character; 5],
}

#[derive(Debug, Error)]
pub enum WordError {
    #[error("Word must be exactly 5 characters, got {0}")]
    InvalidWordLength(usize),
    #[error("Word contains invalid character '{0}'")]
    InvalidWordCharacter(char),
    #[error("Invalid position argument, valid position 0-4 got {0}")]
    InvalidPosition(u8),
}

impl Word {
    fn new(frequency: u32, total_probability: f64, word: &str) -> Result<Self, WordError> {
        let chars: Vec<Character> = word
            .chars()
            .enumerate()
            .map(|(i, ch)| Character::new(ch as u8, i as u8, None, 0))
            .collect();

        let word_array = chars
            .try_into()
            .map_err(|_| WordError::InvalidWordLength(word.len()))?;

        Ok(Word {
            word: word_array,
            frequency,
            total_probability,
        })
    }

    pub fn as_str(&self) -> String {
        self.word.iter().map(|c| c.get_char()).collect()
    }

    pub fn from_bytes(bytes: [u8; 5]) -> Result<Self, WordError> {
        let chars: Vec<Character> = bytes
            .iter()
            .enumerate()
            .map(|(i, &byte)| Character::new(byte, i as u8, None, 0))
            .collect();

        let word_array = chars
            .try_into()
            .map_err(|_| WordError::InvalidWordLength(bytes.len()))?;

        Ok(Word {
            word: word_array,
            frequency: 0,
            total_probability: 0.0,
        })
    }

    fn get_char_at(&self, position: usize) -> Result<char, WordError> {
        if position < 5 {
            Ok(self.word[position].get_char())
        } else {
            Err(WordError::InvalidPosition(position as u8))
        }
    }

    fn contains_char(&self, ch: char) -> bool {
        let byte = ch as u8;
        // Fixed: removed the & since c is already &Character
        self.word.iter().any(|c| c.character == byte)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// WordParser is a state machine for parsing words from a file, It contains a stack parsed of words structs.
pub struct WordParser {
    total_words: u32,
    word_stack: Vec<Word>,
    // we will use the character and the position for the key
    character_hash_map: HashMap<String, Character>, // HashMap to store character frequencies
}

impl WordParser {
    pub fn new() -> Self {
        WordParser {
            total_words: 0,
            word_stack: Vec::new(),
            character_hash_map: HashMap::new(),
        }
    }

    /// Pushes a word onto the Parser's stack
    fn push(&mut self, word: Word) {
        self.word_stack.push(word);
    }

    /// Parses a single word and updates the parsers internal character frequencies map
    pub fn parse_word(&mut self, word: &str) -> Result<(), WordError> {
        //Handle word validation
        if word.len() != 5 {
            return Err(WordError::InvalidWordLength(word.len()));
        }
        if let Some(char) = word.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(WordError::InvalidWordCharacter(char));
        }

        self.total_words += 1;
        for (i, c) in word.chars().enumerate() {
            let key = format!("{}{}", c, i);
            let character = self
                .character_hash_map
                .entry(key)
                .or_insert(Character::new(c as u8, i as u8, None, 0));
            character.increment_frequency();
        }
        self.push(Word::new(0, 0.00, word)?);
        Ok(())
    }

    // this will give a completed hashmap of characters with their respective probabilities based on the list of words.
    pub fn finalize_probabilities(&mut self) {
        // Calculate total frequencies for each position in the five letter word
        let mut position_totals = [0u32; 5];

        for character in self.character_hash_map.values() {
            position_totals[character.position as usize] += character.frequency;
        }

        // Update probabilities based on position-specific totals
        for character in self.character_hash_map.values_mut() {
            let total = position_totals[character.position as usize];
            if total > 0 {
                character.update_probability(total);
            }
        }
    }
    // pop a word from the stack and finish calculating probabilities based of the combined frequencies of characters at each position.
    pub fn pop_n_parse(&mut self) -> Option<Word> {
        match self.word_stack.pop() {
            Some(mut word) => {
                let mut word_probability = 0u32;

                // Calculate total probability for the word based on character frequencies
                for (i, character) in word.word.iter().enumerate() {
                    let key = format!("{}{}", character.get_char(), i);
                    if let Some(char_data) = self.character_hash_map.get(&key) {
                        if let Some(prob) = char_data.probability {
                            word_probability += prob;
                        }
                    }
                }

                // Update the word's total probability (convert percentage to decimal)
                word.total_probability = word_probability as f64 / 100.0;

                Some(word)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let character = Character::new(b'a', 0, None, 5);
        assert_eq!(character.get_char(), 'a');
        assert_eq!(character.position, 0);
        assert_eq!(character.frequency, 5);
        assert_eq!(character.probability, None);
    }

    #[test]
    fn test_character_increment_frequency() {
        let mut character = Character::new(b'a', 0, None, 5);
        character.increment_frequency();
        assert_eq!(character.frequency, 6);
    }

    #[test]
    fn test_character_update_probability() {
        let mut character = Character::new(b'a', 0, None, 25);
        character.update_probability(100);
        assert_eq!(character.probability, Some(25)); // 25/100 * 100 = 25%
    }

    #[test]
    fn test_word_creation_valid() {
        let word = Word::new(1, 0.5, "hello").unwrap();
        assert_eq!(word.as_str(), "hello");
        assert_eq!(word.frequency, 1);
        assert_eq!(word.total_probability, 0.5);
    }

    #[test]
    fn test_word_get_char_at() {
        let word = Word::new(1, 0.5, "hello").unwrap();
        assert_eq!(word.get_char_at(0).unwrap(), 'h');
        assert_eq!(word.get_char_at(4).unwrap(), 'o');

        let result = word.get_char_at(5);
        assert!(matches!(result, Err(WordError::InvalidPosition(5))));
    }

    #[test]
    fn test_word_contains_char() {
        let word = Word::new(1, 0.5, "hello").unwrap();
        assert!(word.contains_char('h'));
        assert!(word.contains_char('e'));
        assert!(word.contains_char('l'));
        assert!(word.contains_char('o'));
        assert!(!word.contains_char('z'));
        assert!(!word.contains_char('a'));
    }

    #[test]
    fn test_word_from_bytes() {
        let bytes = [b'h', b'e', b'l', b'l', b'o'];
        let word = Word::from_bytes(bytes).unwrap();
        assert_eq!(word.as_str(), "hello");
        assert_eq!(word.frequency, 0);
        assert_eq!(word.total_probability, 0.0);
    }

    #[test]
    fn test_word_display() {
        let word = Word::new(1, 0.75, "world").unwrap();
        let display_string = format!("{}", word);
        assert_eq!(display_string, "world");
    }

    #[test]
    fn test_parser_creation() {
        let parser = WordParser::new();
        assert_eq!(parser.total_words, 0);
        assert_eq!(parser.word_stack.len(), 0);
        assert_eq!(parser.character_hash_map.len(), 0);
    }

    #[test]
    fn test_parser_single_word() {
        let mut parser = WordParser::new();
        assert!(parser.parse_word("hello").is_ok());

        assert_eq!(parser.total_words, 1);
        assert_eq!(parser.word_stack.len(), 1);
        assert_eq!(parser.character_hash_map.len(), 5); // h0, e1, l2, l3, o4

        // Check that each character was recorded
        assert!(parser.character_hash_map.contains_key("h0"));
        assert!(parser.character_hash_map.contains_key("e1"));
        assert!(parser.character_hash_map.contains_key("l2"));
        assert!(parser.character_hash_map.contains_key("l3"));
        assert!(parser.character_hash_map.contains_key("o4"));
    }

    #[test]
    fn test_parser_multiple_words() {
        let mut parser = WordParser::new();
        parser.parse_word("hello").unwrap();
        parser.parse_word("world").unwrap();
        parser.parse_word("helps").unwrap();

        assert_eq!(parser.total_words, 3);
        assert_eq!(parser.word_stack.len(), 3);

        // Check frequency of 'h' in position 0 (appears in "hello" and "helps")
        let h0_char = parser.character_hash_map.get("h0").unwrap();
        assert_eq!(h0_char.frequency, 2);

        // Check frequency of 'l' in position 2 (appears in "hello" but NOT "world" - 'r' is in pos 2)
        // "hello" = h0,e1,l2,l3,o4
        // "world" = w0,o1,r2,l3,d4
        // "helps" = h0,e1,l2,p3,s4
        // So 'l' in position 2 appears in "hello" and "helps" = 2 times
        let l2_char = parser.character_hash_map.get("l2").unwrap();
        assert_eq!(l2_char.frequency, 2);

        // Check frequency of 'e' in position 1 (appears in "hello" and "helps")
        let e1_char = parser.character_hash_map.get("e1").unwrap();
        assert_eq!(e1_char.frequency, 2);
    }

    #[test]
    fn test_parser_repeated_letters_same_word() {
        let mut parser = WordParser::new();
        parser.parse_word("llama").unwrap();

        // 'l' appears twice but in different positions
        let l0_char = parser.character_hash_map.get("l0").unwrap();
        assert_eq!(l0_char.frequency, 1);

        let l1_char = parser.character_hash_map.get("l1").unwrap();
        assert_eq!(l1_char.frequency, 1);

        // 'a' appears twice but in different positions
        let a2_char = parser.character_hash_map.get("a2").unwrap();
        assert_eq!(a2_char.frequency, 1);

        let a4_char = parser.character_hash_map.get("a4").unwrap();
        assert_eq!(a4_char.frequency, 1);
    }

    #[test]
    fn test_finalize_probabilities() {
        let mut parser = WordParser::new();
        parser.parse_word("arose").unwrap(); // a0, r1, o2, s3, e4
        parser.parse_word("alert").unwrap(); // a0, l1, e2, r3, t4
        parser.parse_word("above").unwrap(); // a0, b1, o2, v3, e4

        parser.finalize_probabilities();

        // Position 0: 'a' appears 3 times out of 3 words = 100%
        let a0_char = parser.character_hash_map.get("a0").unwrap();
        assert_eq!(a0_char.probability, Some(100));

        // Position 1: 'r', 'l', 'b' each appear 1 time out of 3 = 33%
        let r1_char = parser.character_hash_map.get("r1").unwrap();
        assert_eq!(r1_char.probability, Some(33));

        // Position 4: 'e' appears 2 times out of 3 = 66%
        let e4_char = parser.character_hash_map.get("e4").unwrap();
        assert_eq!(e4_char.probability, Some(66));
    }

    #[test]
    fn test_pop_n_parse_with_probabilities() {
        let mut parser = WordParser::new();
        parser.parse_word("arose").unwrap();
        parser.parse_word("slate").unwrap();

        parser.finalize_probabilities();

        // Pop a word and check its calculated probability
        let word = parser.pop_n_parse().unwrap();
        assert!(word.total_probability > 0.0);
        assert!(word.total_probability <= 5.0); // Max 100% per position * 5 positions / 100

        // Check that we can pop both words
        let word2 = parser.pop_n_parse().unwrap();
        assert!(word2.total_probability > 0.0);

        // Third pop should return None
        assert!(parser.pop_n_parse().is_none());
    }

    #[test]
    fn test_pop_n_parse_empty_stack() {
        let mut parser = WordParser::new();
        parser.finalize_probabilities();

        assert!(parser.pop_n_parse().is_none());
    }

    #[test]
    fn test_parser_case_sensitivity() {
        let mut parser = WordParser::new();

        // Should handle uppercase input
        parser.parse_word("HELLO").unwrap();

        let word = &parser.word_stack[0];
        assert_eq!(word.as_str(), "HELLO");

        // Check that keys are created correctly
        assert!(parser.character_hash_map.contains_key("H0"));
        assert!(parser.character_hash_map.contains_key("E1"));
    }

    #[test]
    fn test_comprehensive_probability_calculation() {
        let mut parser = WordParser::new();

        // Add words with known patterns
        parser.parse_word("tests").unwrap(); // t0, e1, s2, t3, s4
        parser.parse_word("toast").unwrap(); // t0, o1, a2, s3, t4
        parser.parse_word("trait").unwrap(); // t0, r1, a2, i3, t4
        parser.parse_word("twist").unwrap(); // t0, w1, i2, s3, t4

        parser.finalize_probabilities();

        // Position 0: 't' appears 4/4 times = 100%
        let t0 = parser.character_hash_map.get("t0").unwrap();
        assert_eq!(t0.probability, Some(100));

        // Position 3: 't' appears 1/4 times = 25%, 's' appears 2/4 times = 50%
        let t3 = parser.character_hash_map.get("t3").unwrap();
        assert_eq!(t3.probability, Some(25));

        let s3 = parser.character_hash_map.get("s3").unwrap();
        assert_eq!(s3.probability, Some(50));

        // Position 4: 's' appears 1/4 times = 25%, 't' appears 3/4 times = 75%
        let s4 = parser.character_hash_map.get("s4").unwrap();
        assert_eq!(s4.probability, Some(25));

        let t4 = parser.character_hash_map.get("t4").unwrap();
        assert_eq!(t4.probability, Some(75));
    }

    #[test]
    fn test_word_probability_calculation_integration() {
        let mut parser = WordParser::new();

        // Create a scenario where we can predict the probability
        parser.parse_word("aaaaa").unwrap(); // All 'a's
        parser.parse_word("bbbbb").unwrap(); // All 'b's

        parser.finalize_probabilities();

        let word = parser.pop_n_parse().unwrap();

        // Each position should have 50% probability (1 out of 2 words)
        // Total should be 5 * 50% = 250% -> 2.5 when converted to decimal
        assert_eq!(word.total_probability, 2.5);
    }

    #[test]
    fn test_parser_error_handling() {
        let mut parser = WordParser::new();

        // Add some valid words first to test state preservation
        parser.parse_word("hello").unwrap();
        parser.parse_word("world").unwrap();
        parser.parse_word("tests").unwrap();

        let initial_word_count = parser.total_words;
        let initial_stack_len = parser.word_stack.len();
        let initial_hash_len = parser.character_hash_map.len();

        // Test invalid word lengths
        assert!(matches!(
            parser.parse_word("hi"),
            Err(WordError::InvalidWordLength(2))
        ));

        assert!(matches!(
            parser.parse_word("toolong"),
            Err(WordError::InvalidWordLength(7))
        ));

        // Test invalid characters
        assert!(matches!(
            parser.parse_word("he11o"),
            Err(WordError::InvalidWordCharacter('1'))
        ));

        // Parser state should remain unchanged after errors
        assert_eq!(parser.total_words, initial_word_count);
        assert_eq!(parser.word_stack.len(), initial_stack_len);
        assert_eq!(parser.character_hash_map.len(), initial_hash_len);
    }
}
