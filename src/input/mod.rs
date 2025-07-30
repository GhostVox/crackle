mod api;
mod interactive;
mod test;
pub use api::ApiInput;
pub use interactive::InteractiveInput;
pub use test::TestInput;

pub trait InputSource {
    fn get_feedback(&self, word: &str) -> String;
    fn has_next_game(&self) -> bool;
    fn next_game(&mut self);
}
