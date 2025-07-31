use std::io::Write;

use crate::output::OutputSink;
#[allow(dead_code)]
pub struct TestOutput<W: Write> {
    writer: W,
}

#[allow(unused_variables)]
impl<W: Write> OutputSink for TestOutput<W> {
    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        todo!("Implement fatal_error method")
    }

    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        todo!("Implement output_guess method")
    }

    fn out_of_guesses(&mut self) -> Result<(), std::io::Error> {
        todo!("Implement out_of_guesses method")
    }
}
