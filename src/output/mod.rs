mod api;
mod interactive;
mod test;

pub use api::ApiOutput;
pub use interactive::InteractiveOutput;
pub use test::TestOutput;

pub trait OutputSink {
    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error>;
    fn welcome(&mut self, first_guess: &str) -> Result<(), std::io::Error>;
    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error>;
}
