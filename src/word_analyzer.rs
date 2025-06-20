use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub character: u8,
    pub position: u8,
    pub probability: Option<u32>,
    pub frequency: u32,
}

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
    #[error("Probabilities not finalized yet, please call finalize_probabilities")]
    ProbabilitiesNotFinalized,
}

#[derive(Debug, Error)]
pub enum WordAnalyzerError {
    #[error("Probabilities not finalized yet, please call finalize_probabilities")]
    ProbabilitiesNotFinalized,
}

impl Word {
    pub fn new(frequency: u32, total_probability: f64, word: &str) -> Result<Self, WordError> {
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

    // Added missing method
    fn update_probability(&mut self, probability: f64) {
        self.total_probability = probability;
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct WordAnalyzer {
    total_words: u32,
    word_stack: Vec<Word>,
    pub character_hash_map: HashMap<String, Character>,
    probabilitys_finalized: bool,
}

impl WordAnalyzer {
    pub fn new() -> Self {
        WordAnalyzer {
            total_words: 0,
            word_stack: Vec::new(),
            character_hash_map: HashMap::new(),
            probabilitys_finalized: false,
        }
    }

    fn push(&mut self, word: Word) {
        self.word_stack.push(word);
    }

    pub fn pop(&mut self) -> Result<Option<Word>, WordAnalyzerError> {
        if !self.probabilitys_finalized {
            return Err(WordAnalyzerError::ProbabilitiesNotFinalized);
        }
        Ok(self.word_stack.pop())
    }

    pub fn analyze_word(&mut self, word: &str) -> Result<(), WordError> {
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

    pub fn get_total_words(&self) -> u32 {
        self.total_words
    }

    pub fn finalize_probabilities(&mut self) {
        if self.probabilitys_finalized {
            return;
        }

        // Calculate total frequencies for each position
        let mut position_totals = [0u32; 5];
        for character in self.character_hash_map.values() {
            position_totals[character.position as usize] += character.frequency;
        }

        // Update character probabilities
        for character in self.character_hash_map.values_mut() {
            let total = position_totals[character.position as usize];
            if total > 0 {
                character.update_probability(total);
            }
        }

        // Update word probabilities
        for word in &mut self.word_stack {
            let mut word_probability = 0u32;
            for (i, character) in word.word.iter().enumerate() {
                let key = format!("{}{}", character.get_char(), i);
                if let Some(char_data) = self.character_hash_map.get(&key) {
                    if let Some(prob) = char_data.probability {
                        word_probability += prob;
                    }
                }
            }
            word.update_probability(word_probability as f64 / 100.0);
        }

        self.probabilitys_finalized = true;
    }

    // Fixed version of get_most_probable_word
    pub fn get_most_probable_word(&mut self) -> Option<&Word> {
        if !self.probabilitys_finalized {
            self.finalize_probabilities();
        }

        if self.word_stack.is_empty() {
            return None;
        }

        let mut highest_probability_word: Option<&Word> = None;
        for word in self.word_stack.iter() {
            match highest_probability_word {
                None => highest_probability_word = Some(word),
                Some(current_best) => {
                    if word.total_probability > current_best.total_probability {
                        highest_probability_word = Some(word);
                    }
                }
            }
        }
        highest_probability_word
    }

    // Helper method for testing - removed the old pop_n_parse references
    pub fn pop_with_probability(&mut self) -> Result<Option<Word>, WordAnalyzerError> {
        if !self.probabilitys_finalized {
            return Err(WordAnalyzerError::ProbabilitiesNotFinalized);
        }
        Ok(self.word_stack.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Tests for get_most_probable_word ==========

    #[test]
    fn test_get_most_probable_word_empty_analyzer() {
        let mut analyzer = WordAnalyzer::new();
        assert!(analyzer.get_most_probable_word().is_none());
    }

    #[test]
    fn test_get_most_probable_word_single_word() {
        let mut analyzer = WordAnalyzer::new();
        analyzer.analyze_word("hello").unwrap();

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str(), "hello");

        // Should have automatically finalized probabilities
        assert!(analyzer.probabilitys_finalized);
    }

    #[test]
    fn test_get_most_probable_word_auto_finalizes() {
        let mut analyzer = WordAnalyzer::new();
        analyzer.analyze_word("hello").unwrap();
        analyzer.analyze_word("world").unwrap();

        // Probabilities not finalized yet
        assert!(!analyzer.probabilitys_finalized);

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        // Should have automatically finalized
        assert!(analyzer.probabilitys_finalized);
    }

    #[test]
    fn test_get_most_probable_word_clear_winner() {
        let mut analyzer = WordAnalyzer::new();

        // Create words where one will clearly be more probable
        analyzer.analyze_word("aaaaa").unwrap(); // All a's - very high probability
        analyzer.analyze_word("abcde").unwrap(); // Mixed - lower probability
        analyzer.analyze_word("fghij").unwrap(); // All different - lowest probability

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        let most_probable = result.unwrap();
        assert_eq!(most_probable.as_str(), "aaaaa");

        // Verify it has the highest probability
        assert!(most_probable.total_probability > 0.0);
    }

    #[test]
    fn test_get_most_probable_word_realistic_scenario() {
        let mut analyzer = WordAnalyzer::new();

        // Add words with overlapping patterns
        analyzer.analyze_word("tests").unwrap(); // t0, e1, s2, t3, s4
        analyzer.analyze_word("toast").unwrap(); // t0, o1, a2, s3, t4
        analyzer.analyze_word("trust").unwrap(); // t0, r1, u2, s3, t4
        analyzer.analyze_word("twist").unwrap(); // t0, w1, i2, s3, t4

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        let most_probable = result.unwrap();

        // All words start with 't' (100% at position 0)
        // Words ending in 't' should be more probable than "tests"
        // because 't' at position 4 appears 3/4 times (75%) vs 's' at 1/4 times (25%)
        assert_ne!(most_probable.as_str(), "tests");

        // Should be one of the 't' ending words
        let word_str = most_probable.as_str();
        assert!(word_str == "toast" || word_str == "trust" || word_str == "twist");
    }

    #[test]
    fn test_get_most_probable_word_predictable_probabilities() {
        let mut analyzer = WordAnalyzer::new();

        // Scenario with predictable probabilities
        analyzer.analyze_word("aaaaa").unwrap(); // All a's
        analyzer.analyze_word("bbbbb").unwrap(); // All b's

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        let most_probable = result.unwrap();

        // Both words should have equal probability (2.5)
        // Each character has 50% probability, total = 5 * 50% = 250% = 2.5
        assert_eq!(most_probable.total_probability, 2.5);

        // Should return one of the words (implementation choice which one)
        let word_str = most_probable.as_str();
        assert!(word_str == "aaaaa" || word_str == "bbbbb");
    }

    #[test]
    fn test_get_most_probable_word_preserves_state() {
        let mut analyzer = WordAnalyzer::new();
        analyzer.analyze_word("hello").unwrap();
        analyzer.analyze_word("world").unwrap();

        let initial_word_count = analyzer.word_stack.len();
        let initial_total_words = analyzer.total_words;

        // Call once and clone the result string
        let result1_str = analyzer.get_most_probable_word().unwrap().as_str();

        // Call again
        let result2_str = analyzer.get_most_probable_word().unwrap().as_str();

        // Should return same result
        assert_eq!(result1_str, result2_str);

        // Should not modify state
        assert_eq!(analyzer.word_stack.len(), initial_word_count);
        assert_eq!(analyzer.total_words, initial_total_words);
    }

    #[test]
    fn test_get_most_probable_word_with_ties() {
        let mut analyzer = WordAnalyzer::new();

        // Create words with identical patterns (should have same probability)
        analyzer.analyze_word("abcde").unwrap();
        analyzer.analyze_word("fghij").unwrap();

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        // Should return one of them (first one found with highest probability)
        let word_str = result.unwrap().as_str();
        assert!(word_str == "abcde" || word_str == "fghij");
    }

    #[test]
    fn test_get_most_probable_word_complex_scenario() {
        let mut analyzer = WordAnalyzer::new();

        // Mix of high and low probability words
        analyzer.analyze_word("smart").unwrap(); // s0, m1, a2, r3, t4
        analyzer.analyze_word("start").unwrap(); // s0, t1, a2, r3, t4
        analyzer.analyze_word("sport").unwrap(); // s0, p1, o2, r3, t4
        analyzer.analyze_word("shirt").unwrap(); // s0, h1, i2, r3, t4
        analyzer.analyze_word("short").unwrap(); // s0, h1, o2, r3, t4

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        let most_probable = result.unwrap();

        // All start with 's' (100%)
        // All have 'r' at position 3 (100%)
        // All end with 't' (100%)
        // Differences are in positions 1 and 2

        // Words with 'h' at position 1: "shirt", "short" (2/5 = 40%)
        // Words with 'o' at position 2: "sport", "short" (2/5 = 40%)

        // "short" should be most probable as it has common letters in positions 1 and 2
        assert_eq!(most_probable.as_str(), "short");
    }

    #[test]
    fn test_get_most_probable_word_probability_calculation() {
        let mut analyzer = WordAnalyzer::new();

        analyzer.analyze_word("aaaaa").unwrap();
        analyzer.analyze_word("aaaab").unwrap();
        analyzer.analyze_word("aaaac").unwrap();

        let result = analyzer.get_most_probable_word();
        assert!(result.is_some());

        let most_probable = result.unwrap();

        // Position 0-3: 'a' appears 3/3 times = 100% each
        // Position 4: 'a' appears 1/3 times = 33%, 'b' and 'c' appear 1/3 times each = 33%

        // "aaaaa" should have probability: 100 + 100 + 100 + 100 + 33 = 433% = 4.33
        assert_eq!(most_probable.as_str(), "aaaaa");
        assert_eq!(most_probable.total_probability, 4.33);
    }

    // ========== Original tests (keeping the working ones) ==========

    #[test]
    fn test_character_creation() {
        let character = Character::new(b'a', 0, None, 5);
        assert_eq!(character.get_char(), 'a');
        assert_eq!(character.position, 0);
        assert_eq!(character.frequency, 5);
        assert_eq!(character.probability, None);
    }

    #[test]
    fn test_word_creation_valid() {
        let word = Word::new(1, 0.5, "hello").unwrap();
        assert_eq!(word.as_str(), "hello");
        assert_eq!(word.frequency, 1);
        assert_eq!(word.total_probability, 0.5);
    }

    #[test]
    fn test_parser_creation() {
        let parser = WordAnalyzer::new();
        assert_eq!(parser.total_words, 0);
        assert_eq!(parser.word_stack.len(), 0);
        assert_eq!(parser.character_hash_map.len(), 0);
    }

    #[test]
    fn test_parser_single_word() {
        let mut parser = WordAnalyzer::new();
        assert!(parser.analyze_word("hello").is_ok());

        assert_eq!(parser.total_words, 1);
        assert_eq!(parser.word_stack.len(), 1);
        assert_eq!(parser.character_hash_map.len(), 5);
    }

    #[test]
    fn test_finalize_probabilities() {
        let mut parser = WordAnalyzer::new();
        parser.analyze_word("arose").unwrap();
        parser.analyze_word("alert").unwrap();
        parser.analyze_word("above").unwrap();

        parser.finalize_probabilities();

        let a0_char = parser.character_hash_map.get("a0").unwrap();
        assert_eq!(a0_char.probability, Some(100));
    }

    #[test]
    fn test_pop_requires_finalized_probabilities() {
        let mut parser = WordAnalyzer::new();
        parser.analyze_word("hello").unwrap();

        // Should fail before finalization
        assert!(matches!(
            parser.pop(),
            Err(WordAnalyzerError::ProbabilitiesNotFinalized)
        ));

        // Should work after finalization
        parser.finalize_probabilities();
        assert!(parser.pop().unwrap().is_some());
    }
}
