use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct Character {
    character: u8,
    position: u8,
    probability: Option<u32>,
    frequency: u32,
}

/// Struct representing a character in a word, with methods for creating new instances, incrementing frequency, getting character, updating probability.
impl Character {
    pub fn new(character: u8, position: u8, probability: Option<u32>, frequency: u32) -> Self {
        Character {
            character,
            position,
            probability,
            frequency,
        }
    }

    pub fn increment_frequency(&mut self) {
        self.frequency += 1;
    }

    pub fn get_char(&self) -> char {
        self.character as char
    }

    pub fn update_probability(&mut self, total_frequency: u32) {
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
    pub fn new(frequency: u32, total_probability: f64, word: &str) -> Result<Self, WordError> {
        if word.len() != 5 {
            return Err(WordError::InvalidWordLength(word.len()));
        }
        if let Some(char) = word.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(WordError::InvalidWordCharacter(char));
        }

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

    pub fn get_char_at(&self, position: usize) -> Result<char, WordError> {
        if position < 5 {
            Ok(self.word[position].get_char())
        } else {
            Err(WordError::InvalidPosition(position as u8))
        }
    }

    pub fn contains_char(&self, ch: char) -> bool {
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
struct WordParser {
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

    fn push(&mut self, word: Word) {
        self.word_stack.push(word);
    }

    pub fn parse_word(&mut self, word: &str) -> Result<(), WordError> {
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
        // Calculate total frequencies for each position
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
    pub fn pop_parsed_word(&mut self) -> Option<Word> {
        self.word_stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_word() {
        let mut parser = WordParser::new();
        assert!(parser.parse_word("hello").is_ok());
        assert_eq!(parser.word_stack.len(), 1);
        // Fixed: use .get_char() method instead of casting
        assert_eq!(parser.word_stack[0].word[0].get_char(), 'h');
    }

    #[test]
    fn test_word_creation() {
        let word = Word::new(1, 0.5, "hello").unwrap();
        assert_eq!(word.as_str(), "hello");
        assert_eq!(word.get_char_at(0).unwrap(), 'h');
        assert!(word.contains_char('e'));
        assert!(!word.contains_char('z'));
    }

    #[test]
    fn test_character_frequencies() {
        let mut parser = WordParser::new();
        parser.parse_word("hello").unwrap();
        parser.parse_word("world").unwrap();
        parser.parse_word("helps").unwrap();

        parser.finalize_probabilities();

        // Check that 'h' at position 0 has been seen twice
        let h0_key = "h0".to_string();
        if let Some(char_data) = parser.character_hash_map.get(&h0_key) {
            assert_eq!(char_data.frequency, 2); // "hello" and "helps"
        }
    }
}
