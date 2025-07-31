use std::io::Write;

use crate::output::OutputSink;

#[allow(dead_code)]
pub struct ApiOutput<W: Write> {
    writer: W,
}
#[allow(unused_variables)]
impl<W: Write> OutputSink for ApiOutput<W> {
    fn output_guess(&mut self, guess: &str) -> Result<(), std::io::Error> {
        todo!("output_guess")
    }

    fn fatal_error(&mut self, msg: &str) -> Result<(), std::io::Error> {
        todo!("fatal_error")
    }

    fn out_of_guesses(&mut self) -> Result<(), std::io::Error> {
        todo!("out_of_guesses")
    }
}
