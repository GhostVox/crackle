use crate::input::InputSource;
use crate::shared_state::SharedTestState;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct TestInput {
    random_word: String,
    shared_state: Rc<RefCell<SharedTestState>>,
}

impl TestInput {
    pub fn new(random_word: String, feedback: Rc<RefCell<SharedTestState>>) -> Self {
        TestInput {
            random_word,
            shared_state: feedback,
        }
    }
}

impl InputSource for TestInput {
    fn get_feedback(&mut self) -> Result<String, std::io::Error> {
        let state = self.shared_state.borrow();
        let last_guess = state.guesses.last().unwrap();
        Ok(parse_response(self, last_guess))
    }

    fn has_next_game(&self) -> bool {
        false
    }

    fn next_game(&self) {
        todo!()
    }

    fn is_interactive(&self) -> bool {
        false
    }
}
fn parse_response(input: &TestInput, response: &str) -> String {
    // Collect chars into vectors for easier indexing and manipulation.
    let guess_chars: Vec<char> = response.chars().collect();
    let secret_chars: Vec<char> = input.random_word.chars().collect();

    // Ensure words are the same length, adjust as needed for your game's rules.
    if guess_chars.len() != secret_chars.len() {
        return String::new(); // Or handle error
    }

    let len = guess_chars.len();
    let mut result = vec!['n'; len]; // Default to 'n' (gray/no)
    let mut secret_char_counts = HashMap::new();

    // --- First Pass: Find all "green" matches ('g') ---
    // A character is green if it's the correct letter in the correct position.
    for i in 0..len {
        if guess_chars[i] == secret_chars[i] {
            result[i] = 'g';
        } else {
            // Build a frequency map of the remaining secret characters.
            *secret_char_counts.entry(secret_chars[i]).or_insert(0) += 1;
        }
    }

    // --- Second Pass: Find all "yellow" matches ('y') ---
    // A character is yellow if it exists in the word but is in the wrong position.
    for i in 0..len {
        // Skip letters that are already green.
        if result[i] == 'g' {
            continue;
        }

        // Check if this character exists in our map of remaining secret chars.
        if let Some(count) = secret_char_counts.get_mut(&guess_chars[i]) {
            if *count > 0 {
                result[i] = 'y';
                *count -= 1; // Decrement the count so it can't be used again.
            }
        }
    }

    result.into_iter().collect()
}
