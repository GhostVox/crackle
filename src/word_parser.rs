use thiserror::Error;
#[derive(Debug, Clone)]
pub struct Word {
    pub frequency: u32,
    pub total_probability: f64,
    pub word: [u8; 5],
}

#[derive(Debug, Error)]
pub enum WordError {
    #[error("Word must be exactly 5 characters, got {0}")]
    InvalidWordLength(usize),

    #[error("Word contains invalid character '{0}'")]
    InvalidWordCharacter(char),
}
impl Word {
    pub fn new(frequency: u32, total_probability: f64, word: &str) -> Result<Self, WordError> {
        if word.len() != 5 {
            return Err(WordError::InvalidWordLength(word.len()));
        }
        if let Some(char) = word.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(WordError::InvalidWordCharacter(char));
        }
        let mut word_array = [0u8; 5];
        word_array.copy_from_slice(word.as_bytes());
        Ok(Word {
            word: word_array,
            frequency,
            total_probability,
        })
    }
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.word).unwrap()
    }
    pub fn from_bytes(bytes: [u8; 5]) -> Self {
        Word {
            word: bytes,
            frequency: 0,
            total_probability: 0.0,
        }
    }

    pub fn get_char_at(&self, position: usize) -> Option<char> {
        if position < 5 {
            Some(self.word[position] as char)
        } else {
            None
        }
    }
    pub fn contains_char(&self, ch: char) -> bool {
        let byte = ch as u8;
        self.word.contains(&byte)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn parse_word(word: &str) {}
