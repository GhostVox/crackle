mod api;
mod interactive;
mod test;
use crate::constants::WORD_LENGTH;
use crate::error::RecoverableError;
pub use api::ApiInput;
pub use interactive::InteractiveInput;
pub use test::TestInput;

pub trait InputSource {
    fn get_feedback(&mut self) -> Result<String, std::io::Error>;
    fn has_next_game(&self) -> bool;
    fn next_game(&self);
    fn is_interactive(&self) -> bool;
}

fn check_input(input: &str) -> Result<(), RecoverableError> {
    if input.len() != WORD_LENGTH {
        return Err(RecoverableError::InvalidWordLength(input.len()));
    }
    if !input.chars().all(|c| c == 'g' || c == 'y' || c == 'n') {
        return Err(RecoverableError::InvalidInputFormat(String::from(input)));
    }
    Ok(())
}
